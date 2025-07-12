use std::io::{Stdin, Stdout, stdin, stdout};

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
}

// impl Default for AppState {
//     fn default() -> Self {
//         Self::new()
//     }
// }
