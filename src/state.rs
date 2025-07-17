use crate::input::{self, Action};
use crate::termion::raw::IntoRawMode;
use std::io::{Read, StdinLock, StdoutLock, Write, stdin, stdout};
use termion::{color, cursor, raw::RawTerminal};

use crate::{
    scrape::{self, Article},
    util,
};

#[derive(PartialEq)]
enum Mode {
    Select,
    Article,
}

pub struct AppState<'a> {
    stdout: RawTerminal<StdoutLock<'a>>,
    stdin: StdinLock<'a>,
    term_height: usize,
    articles: Vec<Article>,
    max_items: usize,
    titles: Vec<String>,
    selected_row: usize,
    row_offset: usize,
    mode: Mode,
}

impl<'a> AppState<'a> {
    pub fn new() -> Self {
        let (_, term_height) = termion::terminal_size().unwrap();

        let stdout = stdout();
        let stdout = stdout.lock().into_raw_mode().unwrap();
        let stdin = stdin();
        let stdin = stdin.lock();

        let term_height = term_height as usize;
        let articles = scrape::get_items().unwrap();
        let max_items = articles.len();

        let titles = util::articles_to_titles(articles.clone())
            .into_iter()
            .take(max_items)
            .collect::<Vec<String>>();

        let selected_row = 0;
        let row_offset = 0;
        let mode = Mode::Select;

        AppState {
            stdout,
            stdin,
            term_height,
            articles,
            max_items,
            titles,
            selected_row,
            row_offset,
            mode,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
            if self.selected_row + 1 == self.row_offset {
                self.row_offset -= 1;
            }
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_row + 1 < self.max_items {
            self.selected_row += 1;
            if self.selected_row - self.row_offset + 1 > self.term_height {
                self.row_offset += 1;
            }
        }
    }

    pub fn go_top(&mut self) {
        self.selected_row = 0;
        self.row_offset = 0;
    }

    pub fn go_bottom(&mut self) {
        self.selected_row = self.max_items - 1;
        self.row_offset = self.max_items - self.term_height;
    }

    pub fn enter_article(&mut self) {
        self.mode = Mode::Article;

        let href = self.articles[self.selected_row].clone().href;
        let titles = self.titles[self.selected_row].clone();
        AppState::print_article(&mut self.stdout, href, titles);

        self.selected_row = 0;
    }

    pub fn go_back(&mut self) {
        self.mode = Mode::Select;
        self.selected_row = 0;
    }

    pub fn main(&mut self) {
        let _ = write!(self.stdout, "{}", cursor::Hide);

        AppState::print_titles(&mut self.stdout, &self.titles, self.selected_row);
        loop {
            let b = self.stdin.by_ref().bytes().next().unwrap().unwrap();
            let action = input::handle_input(b);

            match action {
                Action::Quit => break,
                Action::MoveUp if self.selected_row > 0 => {
                    self.move_up();
                }
                Action::MoveDown if self.selected_row + 1 < self.max_items => {
                    self.move_down();
                }
                Action::GotoTop => {
                    self.go_top();
                }
                Action::GotoBottom => {
                    self.go_bottom();
                }
                Action::EnterArticle if self.mode == Mode::Select => {
                    self.enter_article();
                }
                Action::GoBack if self.mode == Mode::Article => {
                    self.go_back();
                }
                // TODO: Search
                // Action::Search if mode == Mode::Select =>
                _ => continue,
            }
            if self.mode == Mode::Select {
                let start_idx = self.row_offset;
                let end_idx = std::cmp::min(start_idx + self.term_height, self.max_items);
                let subset_titles = &self.titles[start_idx..end_idx];
                AppState::print_titles(
                    &mut self.stdout,
                    subset_titles,
                    self.selected_row - self.row_offset,
                );
            }

            self.stdout.flush().unwrap();
            write!(self.stdout, "{}", termion::clear::All).unwrap();
        }

        // Cleanup
        let _ = write!(self.stdout, "{}", cursor::Show);
    }

    fn print_titles(stdout: &mut RawTerminal<StdoutLock>, titles: &[String], selected_row: usize) {
        let _ = write!(stdout, "{}", termion::clear::All);
        for (i, title) in titles.iter().enumerate() {
            if i == selected_row {
                write!(
                    stdout,
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
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, i as u16 + 1),
                    title
                )
                .unwrap();
            }
        }
    }

    fn print_article(stdout: &mut RawTerminal<StdoutLock>, url: String, title: String) {
        let _ = write!(stdout, "{}", termion::clear::All);

        let all_text = scrape::get_article(url).unwrap();
        let _ = write!(
            stdout,
            "\r\n{}{}{}",
            termion::style::Bold,
            title,
            termion::style::Reset
        );

        for text in all_text {
            let _ = write!(stdout, "\r\n{}\r\n", text);
        }
    }
}
