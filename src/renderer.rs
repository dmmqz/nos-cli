use crate::termion::raw::IntoRawMode;
use std::io::{Read, StdinLock, StdoutLock, Write, stdin, stdout};
use termion::{color, cursor, event::Key, input::TermRead, raw::RawTerminal};

pub struct Renderer<'a> {
    stdout: RawTerminal<StdoutLock<'a>>, // TODO: look into AlternateScreen
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
    pub fn print_titles(&mut self, titles: &[String], selected_row: usize, term_height: usize) {
        self.clear_main(term_height);
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
        self.flush();
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
        self.flush();
    }

    pub fn write_string(&mut self, string: String, y_pos: usize) {
        write!(
            self.stdout,
            "{}{}{}",
            termion::cursor::Goto(1, y_pos as u16),
            termion::clear::AfterCursor,
            string
        )
        .unwrap();
        self.flush();
    }

    pub fn get_keystroke(&mut self) -> Key {
        self.stdin.by_ref().keys().next().unwrap().unwrap()
    }

    pub fn clear_all(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
    }

    pub fn clear_status_bar(&mut self, y_pos: usize) {
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(1, y_pos as u16 + 1),
            termion::clear::CurrentLine
        )
        .unwrap();
    }

    fn clear_main(&mut self, y_pos: usize) {
        for i in 0..=y_pos {
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(1, i as u16),
                termion::clear::CurrentLine
            )
            .unwrap();
        }
    }

    pub fn hide_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
    }

    pub fn show_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Show).unwrap();
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}
