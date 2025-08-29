use rand::Rng;
use regex::Regex;

use crate::{
    scrape::{self, Article},
    util,
};

#[derive(PartialEq)]
pub enum Mode {
    Select,
    Article,
}

pub struct State {
    articles: Vec<Article>,
    all_articles: Vec<Article>,
    titles: Vec<String>,
    selected_row: usize,
    row_offset: usize,
    pub mode: Mode, // TODO: use setter/getter
    current_article_text: Vec<String>,
    term_height: usize, // TODO: maybe create trait to refresh this
    term_width: usize,
}

impl State {
    pub fn new(articles: Vec<Article>) -> Self {
        let all_articles = articles.clone();

        let titles = util::articles_to_titles(&articles)
            .into_iter()
            .take(articles.len())
            .collect::<Vec<String>>();

        let selected_row = 0;
        let row_offset = 0;
        let mode = Mode::Select;

        let current_article_text = Vec::new();

        let (term_width, term_height) = termion::terminal_size().unwrap();
        let term_height = term_height as usize - 1;
        let term_width = term_width as usize;

        State {
            articles,
            all_articles,
            titles,
            selected_row,
            row_offset,
            mode,
            current_article_text,
            term_width,
            term_height,
        }
    }

    pub fn move_up(&mut self) {
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

    pub fn move_down(&mut self) {
        match self.mode {
            Mode::Select => {
                if self.selected_row + 1 >= self.articles.len() {
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

    pub fn page_up(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(self.term_height);
        self.row_offset = self.selected_row.saturating_sub(self.term_height - 1);
    }

    pub fn page_down(&mut self) {
        match self.mode {
            Mode::Select => {
                self.selected_row += self.term_height;
                if self.selected_row > self.articles.len() {
                    self.selected_row = self.articles.len() - 1;
                }
                self.row_offset =
                    std::cmp::min(self.selected_row, self.articles.len() - self.term_height);
            }
            Mode::Article => {
                if self.row_offset + self.term_height >= self.current_article_text.len() {
                    return;
                }
                self.row_offset += std::cmp::min(
                    self.term_height,
                    self.current_article_text.len() - self.term_height,
                )
            }
        }
    }

    pub fn go_top(&mut self) {
        self.row_offset = 0;
        if self.mode == Mode::Select {
            self.selected_row = 0;
        }
    }

    pub fn go_bottom(&mut self) {
        match self.mode {
            Mode::Select => {
                self.selected_row = self.articles.len() - 1;
                self.row_offset = self.articles.len().saturating_sub(self.term_height);
            }
            Mode::Article => {
                self.row_offset = self
                    .current_article_text
                    .len()
                    .saturating_sub(self.term_height);
            }
        }
    }

    pub fn enter_article(&mut self) {
        self.mode = Mode::Article;

        let url = self.articles[self.selected_row].href.as_str();
        let raw_article_text =
            scrape::get_article(url).expect("Request for getting the article failed.");

        let mut formatted_article_text: Vec<String> = Vec::new();
        for line in textwrap::wrap(
            &self.articles[self.selected_row].title.clone(),
            self.term_width,
        ) {
            formatted_article_text.push(line.to_string());
        }

        for text in raw_article_text {
            let wrapped_text = textwrap::wrap(&text, self.term_width);
            for line in wrapped_text {
                formatted_article_text.push(format!("\r\n{}", line.to_string()));
            }
            formatted_article_text.push("\r\n".to_string())
        }

        self.current_article_text = formatted_article_text;

        self.go_top();
    }

    pub fn go_back(&mut self) {
        if !(self.mode == Mode::Article) {
            return;
        }
        self.mode = Mode::Select;
        self.go_top();
    }

    pub fn reset(&mut self) {
        self.articles = self.all_articles.clone();
        self.titles = util::articles_to_titles(&self.articles)
            .into_iter()
            .take(self.articles.len())
            .collect::<Vec<String>>();
        self.go_top();
    }

    pub fn filter_articles(&mut self, search_string: &str) -> Vec<String> {
        self.reset();
        let re = Regex::new(search_string).unwrap_or(Regex::new("").unwrap());

        let mut matches: Vec<Article> = Vec::new();
        for (i, title) in self.titles.iter().enumerate() {
            if re.is_match(&title.to_lowercase()) {
                matches.push(self.all_articles[i].clone());
            }
        }
        self.articles = matches;
        self.titles = util::articles_to_titles(&self.articles)
            .into_iter()
            .take(self.articles.len())
            .collect::<Vec<String>>();

        self.get_subset().to_owned()
    }

    pub fn get_subset(&self) -> &[String] {
        let start_idx = self.row_offset;

        match self.mode {
            Mode::Select => {
                let end_idx = std::cmp::min(start_idx + self.term_height, self.articles.len());
                return &self.titles[start_idx..end_idx];
            }
            Mode::Article => {
                let end_idx = std::cmp::min(
                    start_idx + self.term_height,
                    self.current_article_text.len(),
                );
                return &self.current_article_text[start_idx..end_idx - 1];
            }
        }
    }

    pub fn get_relative_row(&self) -> usize {
        self.selected_row - self.row_offset
    }

    pub fn random_article(&mut self) {
        if self.articles.len() == 0 {
            self.reset();
        }
        self.selected_row = rand::rng().random_range(0..self.articles.len());
        self.enter_article();
    }
}
