use crate::termion::raw::IntoRawMode;
use std::io::{Read, StdinLock, StdoutLock, Write, stdin, stdout};
use termion::{color, cursor, raw::RawTerminal};

pub struct Renderer<'a> {
    stdout: RawTerminal<StdoutLock<'a>>,
    stdin: StdinLock<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        let stdout = stdout();
        let stdout = stdout.lock().into_raw_mode().unwrap();
        let stdin = stdin();
        let stdin = stdin.lock();

        Renderer {
            stdout: stdout,
            stdin: stdin,
        }
    }
    pub fn print_titles(&mut self, titles: &[String], selected_row: usize) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        for (i, title) in titles.iter().enumerate() {
            if i == selected_row {
                write!(
                    self.stdout,
                    "{}{}{}{}{}{}",
                    termion::cursor::Goto(1, i as u16 + 1),
                    color::Bg(color::White),
                    color::Fg(color::Black),
                    title,
                    color::Fg(color::Reset),
                    color::Bg(color::Reset),
                )
                .unwrap();
            } else {
                write!(
                    self.stdout,
                    "{}{}",
                    termion::cursor::Goto(1, i as u16 + 1),
                    title
                )
                .unwrap();
            }
        }
    }

    pub fn print_article(&mut self, subset_article: &[String]) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();

        for (i, line) in subset_article.iter().enumerate() {
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(1, (i + 1) as u16),
                line
            )
            .unwrap();
        }
    }

    pub fn get_keystroke(&mut self) -> u8 {
        self.stdin
            .by_ref()
            .bytes()
            .next()
            .unwrap()
            .expect("Couldn't read input.")
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
    }

    pub fn hide_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
    }

    pub fn show_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Show).unwrap();
    }
}
