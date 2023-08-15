use crate::{
    cursor::Cursor,
    directory::Directory,
    mode::{Mode, Modes},
    terminal::Terminal,
};
use termion::{color, event::Key};

pub struct Explorer {
    directory: Directory,
    cursor: Cursor,
    terminal: Terminal,
    mode: Mode,
    should_quit: bool,
    offset: usize,
    input: String,
}

impl Default for Explorer {
    fn default() -> Self {
        let mut directory = Directory::new();
        directory.refresh();
        Self {
            cursor: Cursor::from(&directory),
            directory,
            terminal: Terminal::default(),
            mode: Mode::default(),
            should_quit: false,
            offset: 0,
            input: String::new(),
        }
    }
}

impl Explorer {
    pub fn run(&mut self) {
        Terminal::clear_screen();
        Terminal::cursor_hide();

        loop {
            self.refresh_screen();

            if self.should_quit {
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

            if self.cursor.position == i {
                print!("{}", color::Bg(color::Blue));
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
            Modes::Input => print!("{}", self.input),
            _ => print!("{}", self.mode),
        };
    }

    pub fn handle_keypress(&mut self) {
        let key = Terminal::read_input().unwrap();
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Char('k') | Key::Up => self.cursor.mut_move_rel(-1),
            Key::Char('j') | Key::Down => self.cursor.mut_move_rel(1),
            Key::Char('l') | Key::Right => self.cd_subdir(),
            Key::Char('h') | Key::Left => self.cd_parent(),
            Key::Char('r') => self.prompt(),
            Key::Char('x') => self.directory.delete_item(self.cursor.position),
            _ => {}
        }
        self.scroll();
    }

    pub fn cd_subdir(&mut self) {
        let item = self.directory.item_at(self.cursor.position).unwrap();

        if item.meta.is_dir() {
            self.directory.cd(&item.entry.path());
            self.cursor.update(&self.directory);
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

    pub fn prompt(&mut self) {
        self.mode.switch(Modes::Input);
        Terminal::cursor_show();
        Terminal::cursor_goto(1, self.terminal.height as u16);

        loop {
            self.refresh_screen();
            let key = Terminal::read_input().unwrap();
            match key {
                Key::Char('\n') => break,
                Key::Char(c) => self.input.push(c),
                _ => {}
            }
        }

        Terminal::cursor_hide();
        Terminal::cursor_goto(1, 1);
        self.mode.switch(Modes::Explore);
    }
}
