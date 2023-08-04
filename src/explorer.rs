use crate::{cursor::Cursor, directory::Directory, terminal::Terminal};
use termion::{color, event::Key};

pub struct Explorer {
    directory: Directory,
    cursor: Cursor,
    terminal: Terminal,
    should_quit: bool,
    offset: usize,
}

impl Default for Explorer {
    fn default() -> Self {
        let mut directory = Directory::new();
        directory.refresh();
        Self {
            cursor: Cursor::from(&directory),
            directory,
            terminal: Terminal::default(),
            should_quit: false,
            offset: 0,
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
        if self.cursor.position >= self.offset + self.terminal.height - 3 {
            self.offset += 1;
        }

        if self.cursor.position < self.offset {
            self.offset -= 1;
        }
    }

    fn refresh_screen(&mut self) {
        Terminal::cursor_goto(1, 1);
        Terminal::clear_after_cursor();
        println!("{:?}\r", self.directory.path);
        println!(
            "{}/{} || {} {} {}\r",
            self.cursor.position + 1,
            self.cursor.max + 1,
            self.cursor.position,
            self.offset,
            self.terminal.height
        );

        for i in self.offset..self.terminal.height - 3 + self.offset {
            let item = match self.directory.item_at(i) {
                Some(item) => item,
                None => break,
            };

            let meta = item.metadata().unwrap();
            let path = item.path();
            let display = path.file_name().unwrap();

            if self.cursor.position == i {
                print!("{}", color::Bg(color::Blue));
            }

            println!(
                "{} {}{}\r",
                if meta.is_dir() { "[D]" } else { "[F]" },
                display.to_str().unwrap(),
                " ".repeat(self.terminal.width - 4 - display.len()),
            );

            print!("{}", color::Bg(color::Reset));
        }
    }

    pub fn handle_keypress(&mut self) {
        let key = Terminal::read_input().unwrap();
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Char('k') | Key::Up => self.cursor.mut_move_relative(-1),
            Key::Char('j') | Key::Down => self.cursor.mut_move_relative(1),
            Key::Char('l') | Key::Right => self.cd_subdir(),
            Key::Char('h') | Key::Left => self.cd_parent(),
            _ => {}
        }
        self.scroll();
    }

    pub fn cd_subdir(&mut self) {
        let item = self.directory.item_at(self.cursor.position).unwrap();
        let meta = item.metadata().unwrap();

        if meta.is_dir() {
            self.directory.cd(item.path());
            self.cursor.update(&self.directory);
        }
    }

    pub fn cd_parent(&mut self) {
        let parent = self.directory.path.parent().unwrap().to_path_buf();
        self.directory.cd(parent);
        self.cursor.update(&self.directory);
    }
}
