use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Terminal {
    pub width: usize,
    pub height: usize,
    _stdout: RawTerminal<io::Stdout>,
}

impl Default for Terminal {
    fn default() -> Self {
        let size = termion::terminal_size().unwrap();
        Self {
            width: size.0 as usize,
            height: size.1 as usize,
            _stdout: io::stdout().into_raw_mode().unwrap(),
        }
    }
}

impl Terminal {
    pub fn read_input() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_after_cursor() {
        print!("{}", termion::clear::AfterCursor);
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn cursor_goto(x: u16, y: u16) {
        print!("{}", termion::cursor::Goto(x, y));
    }
}
