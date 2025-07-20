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

pub struct App<'a> {
    stdout: RawTerminal<StdoutLock<'a>>,
    stdin: StdinLock<'a>,
    term_height: usize,
    term_width: usize,
    articles: Vec<Article>,
    max_items: usize,
    titles: Vec<String>,
    selected_row: usize,
    row_offset: usize,
    mode: Mode,
    current_article_text: Vec<String>,
}

impl<'a> App<'a> {
    // TODO: split this struct up, e.g. Render, AppState structs
    pub fn new() -> Self {
        let (term_width, term_height) = termion::terminal_size().unwrap();

        let stdout = stdout();
        let stdout = stdout.lock().into_raw_mode().unwrap();
        let stdin = stdin();
        let stdin = stdin.lock();

        let term_width = term_width as usize;
        let term_height = term_height as usize;
        let articles = scrape::get_items().expect("Couldn't get article titles.");
        let max_items = articles.len();

        let titles = util::articles_to_titles(articles.clone())
            .into_iter()
            .take(max_items)
            .collect::<Vec<String>>();

        let selected_row = 0;
        let row_offset = 0;
        let mode = Mode::Select;

        let current_article_text = Vec::new();

        App {
            stdout,
            stdin,
            term_height,
            term_width,
            articles,
            max_items,
            titles,
            selected_row,
            row_offset,
            mode,
            current_article_text,
        }
    }

    fn move_up(&mut self) {
        match self.mode {
            Mode::Select => {
                if self.selected_row == 0 {
                    return;
                }
                self.selected_row -= 1;
                if self.selected_row + 1 == self.row_offset {
                    self.row_offset -= 1;
                }
            }
            Mode::Article => {
                if self.row_offset == 0 {
                    return;
                }
                self.row_offset -= 1;
            }
        }
    }

    fn move_down(&mut self) {
        match self.mode {
            Mode::Select => {
                if self.selected_row + 1 >= self.max_items {
                    return;
                }
                self.selected_row += 1;
                if self.selected_row - self.row_offset + 1 > self.term_height {
                    self.row_offset += 1;
                }
            }
            Mode::Article => {
                if self.row_offset + self.term_height >= self.current_article_text.len() {
                    return;
                }
                self.row_offset += 1;
            }
        }
    }

    fn go_top(&mut self) {
        self.row_offset = 0;
        if self.mode == Mode::Select {
            self.selected_row = 0;
        }
    }

    fn go_bottom(&mut self) {
        self.selected_row = self.max_items - 1;
        self.row_offset = self.max_items - self.term_height;
    }

    fn enter_article(&mut self) {
        if !(self.mode == Mode::Select) {
            return;
        }
        self.mode = Mode::Article;

        let url = self.articles[self.selected_row].clone().href;
        let raw_article_text =
            scrape::get_article(url).expect("Request for getting the article failed.");

        let mut formatted_article_text: Vec<String> = Vec::new();
        formatted_article_text.push(self.titles[self.selected_row].clone());

        for text in raw_article_text {
            let wrapped_text = textwrap::wrap(&text, self.term_width);
            for line in wrapped_text {
                formatted_article_text.push(format!("\r\n{}", line.to_string()));
            }
            formatted_article_text.push("\r\n".to_string())
        }

        self.current_article_text = formatted_article_text;

        self.go_top();
        self.print_article();
    }

    fn go_back(&mut self) {
        if !(self.mode == Mode::Article) {
            return;
        }
        self.mode = Mode::Select;
        self.go_top();
    }

    pub fn main(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();

        App::print_titles(&mut self.stdout, &self.titles, self.selected_row);
        loop {
            let b = self
                .stdin
                .by_ref()
                .bytes()
                .next()
                .unwrap()
                .expect("Couldn't read input.");
            let action = input::handle_input(b);

            match action {
                Action::Quit => break,
                Action::MoveUp => self.move_up(),
                Action::MoveDown => self.move_down(),
                Action::GotoTop => self.go_top(),
                Action::GotoBottom => self.go_bottom(),
                Action::EnterArticle => self.enter_article(),
                Action::GoBack => self.go_back(),
                // TODO: Search
                // Action::Search if mode == Mode::Select =>
                // TODO: command mode (help, statusbar, etc.)
                _ => continue,
            }
            match self.mode {
                Mode::Select => {
                    let start_idx = self.row_offset;
                    let end_idx = std::cmp::min(start_idx + self.term_height, self.max_items);
                    let subset_titles = &self.titles[start_idx..end_idx];
                    App::print_titles(
                        &mut self.stdout,
                        subset_titles,
                        self.selected_row - self.row_offset,
                    );
                }
                Mode::Article => {
                    self.print_article();
                }
            }

            self.stdout.flush().unwrap();
            write!(self.stdout, "{}", termion::clear::All).unwrap();
        }

        write!(self.stdout, "{}", cursor::Show).unwrap();
    }

    fn print_titles(stdout: &mut RawTerminal<StdoutLock>, titles: &[String], selected_row: usize) {
        write!(stdout, "{}", termion::clear::All).unwrap();
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

    fn print_article(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();

        let start_idx = self.row_offset;
        let end_idx = std::cmp::min(
            start_idx + self.term_height,
            self.current_article_text.len(),
        );
        let subset_article = &self.current_article_text[start_idx..end_idx - 1];

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
}
