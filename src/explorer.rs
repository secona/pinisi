use std::{cell::RefCell, rc::Rc};

use crate::{
    cursor::Cursor,
    directory::Directory,
    mode::{Mode, Modes, Selection},
    terminal::Terminal,
};
use termion::{color, event::Key};

pub struct Explorer {
    directory: Directory,
    cursor: Cursor,
    terminal: Terminal,
    mode: Mode,
    offset: usize,
    input: String,
    cursor_text: Cursor,
}

impl Default for Explorer {
    fn default() -> Self {
        let directory = Directory::default();
        let terminal = Terminal::default();

        Self {
            cursor: Cursor::from(&directory),
            cursor_text: Cursor::new(0, terminal.width),
            mode: Mode::default(),
            input: String::new(),
            offset: 0,
            directory,
            terminal,
        }
    }
}

impl Explorer {
    pub fn run(&mut self) {
        Terminal::clear_screen();
        Terminal::cursor_hide();

        loop {
            self.refresh_screen();

            if matches!(self.mode.get(), Modes::Quit) {
                break;
            }

            self.handle_keypress();
        }

        Terminal::cursor_show();
    }

    fn scroll(&mut self) {
        if self.cursor.position >= self.offset + self.terminal.height - 2 {
            self.offset += 1;
        }

        if self.cursor.position < self.offset {
            self.offset -= 1;
        }
    }

    fn refresh_screen(&mut self) {
        Terminal::cursor_goto(1, 1);
        Terminal::clear_after_cursor();

        self.print_items();
        self.print_status();
        self.print_message();

        Terminal::cursor_goto(
            (self.cursor_text.position + 1) as u16,
            self.terminal.height as u16,
        );
        Terminal::flush().unwrap();
    }

    fn print_items(&self) {
        for i in self.offset..self.terminal.height - 2 + self.offset {
            let item = match self.directory.item_at(i) {
                Some(item) => item,
                None => {
                    println!("\r");
                    continue;
                }
            };

            let path = item.entry.path();
            let display = path.file_name().unwrap();

            match self.mode.get() {
                Modes::Select(selection) => {
                    if self.cursor.position == i {
                        print!("{}", color::Bg(color::Yellow));
                    } else if selection.borrow().selected.contains(&i) {
                        print!("{}", color::Bg(color::LightYellow));
                    }
                }
                _ => {
                    if self.cursor.position == i {
                        print!("{}", color::Bg(color::Blue));
                    }
                }
            }

            println!(
                "{} {}{}\r",
                if item.meta.is_dir() { "[D]" } else { "[F]" },
                display.to_str().unwrap(),
                " ".repeat(self.terminal.width - 4 - display.len()),
            );

            print!("{}", color::Bg(color::Reset));
        }
    }

    fn print_status(&self) {
        let status = format!(
            "[{:?}] [{}/{}]",
            self.directory.path,
            self.cursor.position + 1,
            self.cursor.max + 1,
        );

        println!(
            "{}{}{}{}{}{}\r",
            color::Bg(color::Rgb(239, 239, 239)),
            color::Fg(color::Black),
            status,
            " ".repeat(self.terminal.width - status.len()),
            color::Bg(color::Reset),
            color::Fg(color::Reset)
        );
    }

    fn print_message(&self) {
        match self.mode.get() {
            Modes::Input(_) => print!("{}", self.input),
            _ => print!("{}", self.mode),
        };
    }

    pub fn handle_keypress(&mut self) {
        let key = Terminal::read_input().unwrap();
        match self.mode.get().clone() {
            Modes::Explore => match key {
                Key::Ctrl('q') => self.mode.switch(Modes::Quit),
                Key::Char('k') | Key::Up => self.cursor.mut_move_rel(-1),
                Key::Char('j') | Key::Down => self.cursor.mut_move_rel(1),
                Key::Char('l') | Key::Right => self.cd_subdir(),
                Key::Char('h') | Key::Left => self.cd_parent(),
                Key::Char('r') => {
                    let item = self.directory.item_at(self.cursor.position).unwrap();
                    let default_value = item.entry.path();
                    let result = self.prompt(Some(default_value.to_str().unwrap()));

                    if let Some(value) = result {
                        self.directory.rename_item(&self.cursor.position, value);
                    }
                }
                Key::Char('s') => {
                    self.mode
                        .switch(Modes::Select(Rc::new(RefCell::new(Selection::new(
                            self.cursor.position,
                        )))));
                }
                Key::Char('x') => {
                    self.directory.delete_item(&self.cursor.position);
                    self.cursor.update(&self.directory);
                }
                _ => {}
            },
            Modes::Select(selection) => match key {
                Key::Char('k') | Key::Up => {
                    self.cursor.mut_move_rel(-1);
                    selection.borrow_mut().update(&self.cursor.position);
                }
                Key::Char('j') | Key::Down => {
                    self.cursor.mut_move_rel(1);
                    selection.borrow_mut().update(&self.cursor.position);
                }
                Key::Char('s') => {
                    self.mode.switch(Modes::Explore);
                }
                Key::Char('x') => {
                    for index in &selection.borrow().selected {
                        self.directory.delete_item(index);
                    }
                    self.mode.switch(Modes::Explore);
                    self.cursor.mut_move_abs(0);
                }
                _ => {}
            },
            _ => {}
        }

        self.scroll();
    }

    pub fn cd_subdir(&mut self) {
        match self.directory.item_at(self.cursor.position) {
            None => {}
            Some(item) => {
                if item.meta.is_dir() {
                    self.directory.cd(&item.entry.path());
                    self.cursor.update(&self.directory);
                }
            }
        }
    }

    pub fn cd_parent(&mut self) {
        let path = self.directory.path.clone();

        let parent = match path.parent() {
            Some(parent) => parent,
            None => return,
        };

        self.directory.cd(parent);
        self.cursor.update(&self.directory);

        let index = self
            .directory
            .items
            .iter()
            .position(|item| item.entry.path() == path)
            .unwrap_or(0);

        self.cursor.mut_move_abs(index);
    }

    pub fn prompt(&mut self, default: Option<&str>) -> Option<String> {
        if let Some(value) = default {
            self.input = String::from(value);
        }

        let return_value: Option<String>;
        // self.mode.switch(Modes::Input);
        self.cursor_text.mut_move_abs(self.input.len());
        Terminal::cursor_show();

        loop {
            self.refresh_screen();
            let key = Terminal::read_input().unwrap();
            match key {
                Key::Char('\n') => {
                    return_value = Some(self.input.clone());
                    break;
                }
                Key::Esc => {
                    return_value = None;
                    break;
                }
                Key::Delete => {
                    if self.cursor_text.position < self.input.len() {
                        self.input.remove(self.cursor_text.position);
                    }
                }
                Key::Backspace => {
                    if self.cursor_text.position > 0 {
                        self.input.remove(self.cursor_text.position - 1);
                        self.cursor_text.mut_move_rel(-1);
                    }
                }
                Key::Char(c) => {
                    self.input.insert(self.cursor_text.position, c);
                    self.cursor_text.mut_move_rel(1);
                }
                Key::Left => {
                    if self.cursor_text.position >= 1 {
                        self.cursor_text.mut_move_rel(-1);
                    }
                }
                Key::Right => {
                    if (self.cursor_text.position) < self.input.len() {
                        self.cursor_text.mut_move_rel(1);
                    }
                }
                _ => {}
            }
        }

        Terminal::cursor_hide();
        Terminal::cursor_goto(1, 1);
        self.mode.switch(Modes::Explore);
        self.input = String::new();
        return_value
    }
}
