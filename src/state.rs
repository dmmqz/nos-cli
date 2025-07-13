use std::io::{Stdin, Stdout, Write, stdin, stdout};
use termion::color;

use crate::{
    scrape::{self, Article},
    util,
};

#[derive(PartialEq)]
enum Mode {
    Select,
    Article,
}

pub struct AppState {
    stdout: Stdout,
    stdin: Stdin,
    term_height: usize,
    articles: Vec<Article>,
    max_items: usize,
    titles: Vec<String>,
    selected_row: usize,
    row_offset: usize,
    mode: Mode,
}

impl AppState {
    fn new() -> Self {
        let (_, term_height) = termion::terminal_size().unwrap();

        let stdout = stdout();
        let stdin = stdin();
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

    fn move_up(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
            if self.selected_row + 1 == self.row_offset {
                self.row_offset -= 1;
            }
        }
    }

    fn move_down(&mut self) {
        if self.selected_row + 1 < self.max_items {
            self.selected_row += 1;
            if self.selected_row - self.row_offset + 1 > self.term_height {
                self.row_offset += 1;
            }
        }
    }

    fn go_top(&mut self) {
        self.selected_row = 0;
        self.row_offset = 0;
    }

    fn go_bottom(&mut self) {
        self.selected_row = self.max_items - 1;
        self.row_offset = self.max_items - self.term_height;
    }

    fn enter_article(&mut self) {
        self.mode = Mode::Article;

        let href = self.articles[self.selected_row].clone().href;
        let titles = self.titles[self.selected_row].clone();
        AppState::print_article(&mut self.stdout, href, titles);

        self.selected_row = 0;
    }

    fn print_titles<W: Write>(stdout: &mut W, titles: &[String], selected_row: usize) {
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

    fn print_article<W: Write>(stdout: &mut W, url: String, title: String) {
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
