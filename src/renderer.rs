use crate::termion::raw::IntoRawMode;
use std::io::{Read, StdinLock, StdoutLock, Write, stdin, stdout};
use termion::{color, cursor, event::Key, input::TermRead, raw::RawTerminal};

pub struct Renderer<'a> {
    stdout: RawTerminal<StdoutLock<'a>>, // TODO: look into AlternateScreen
    stdin: StdinLock<'a>,
    term_height: usize,
}

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        let stdout = stdout();
        let stdout = stdout.lock().into_raw_mode().unwrap();
        let stdin = stdin();
        let stdin = stdin.lock();

        let (_, term_height) = termion::terminal_size().unwrap();
        let term_height = term_height as usize - 1;

        Renderer {
            stdout,
            stdin,
            term_height,
        }
    }

    pub fn print_titles(&mut self, titles: &[String], selected_row: usize) {
        self.clear_main();
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
        self.clear_main();

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

    pub fn write_string(&mut self, string: String) {
        write!(
            self.stdout,
            "{}{}{}",
            termion::cursor::Goto(1, self.term_height as u16 + 1),
            termion::clear::AfterCursor,
            string
        )
        .unwrap();
        self.flush();
    }

    pub fn write_error_string(&mut self, string: String) {
        write!(
            self.stdout,
            "{}{}{}{}{}",
            termion::cursor::Goto(1, self.term_height as u16 + 1),
            termion::clear::AfterCursor,
            color::Fg(color::Red),
            string,
            color::Fg(color::Reset),
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

    pub fn clear_status_bar(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(1, self.term_height as u16 + 1),
            termion::clear::CurrentLine
        )
        .unwrap();
    }

    fn clear_main(&mut self) {
        for i in 0..=self.term_height {
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
