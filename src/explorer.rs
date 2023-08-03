use crate::{cursor::Cursor, directory::Directory, terminal::Terminal};
use termion::{color, event::Key};

pub struct Explorer {
    directory: Directory,
    cursor: Cursor,
    terminal: Terminal,
    should_quit: bool,
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

    fn refresh_screen(&mut self) {
        Terminal::cursor_goto(1, 1);

        for (i, item) in self.directory.items.iter().enumerate() {
            let meta = item.metadata().unwrap();
            let display = item.path().display().to_string();

            if self.cursor.position == i {
                print!("{}", color::Bg(color::Blue));
            }

            println!(
                "{} {}{}\r",
                if meta.is_dir() { "DIR" } else { "   " },
                display,
                " ".repeat(self.terminal.width - 4 - display.len()),
            );

            print!("{}", color::Bg(color::Reset));
        }
    }

    pub fn handle_keypress(&mut self) {
        let key = Terminal::read_input().unwrap();
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => {}
        }
    }
}
