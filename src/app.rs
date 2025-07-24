use regex::Regex;
use termion::event::Key;

use crate::{
    input::{self, Action},
    renderer::Renderer,
    scrape::{self, Article},
    util,
};

#[derive(PartialEq)]
enum Mode {
    Select,
    Article,
}

pub struct App {
    renderer: Renderer<'static>,
    term_height: usize,
    term_width: usize,
    articles: Vec<Article>,
    all_articles: Vec<Article>,
    titles: Vec<String>,
    selected_row: usize,
    row_offset: usize,
    mode: Mode,
    current_article_text: Vec<String>,
}

impl App {
    // TODO: split this struct up, e.g. Render, AppState structs
    pub fn new(url: Option<String>) -> Self {
        let (term_width, term_height) = termion::terminal_size().unwrap();
        let link = url.unwrap_or(String::from("https://nos.nl/nieuws/laatste"));

        let renderer = Renderer::new();

        let term_width = term_width as usize;
        let term_height = term_height as usize - 1;
        let articles = scrape::get_items(link).expect("Couldn't get article titles.");
        let all_articles = articles.clone();
        let max_items = articles.len();

        let titles = util::articles_to_titles(&articles)
            .into_iter()
            .take(max_items)
            .collect::<Vec<String>>();

        let selected_row = 0;
        let row_offset = 0;
        let mode = Mode::Select;

        let current_article_text = Vec::new();

        App {
            renderer,
            term_height,
            term_width,
            articles,
            all_articles,
            titles,
            selected_row,
            row_offset,
            mode,
            current_article_text,
        }
    }

    pub fn main(&mut self) {
        self.renderer.hide_cursor();

        let subset_titles = self.get_subset().to_owned();
        self.renderer.print_titles(
            &subset_titles,
            self.selected_row - self.row_offset,
            self.term_height,
        );
        loop {
            let keystroke = self.renderer.get_keystroke();
            let action = input::handle_input(keystroke);

            match action {
                Action::Quit => break,
                Action::MoveUp => self.move_up(),
                Action::MoveDown => self.move_down(),
                Action::GotoTop => self.go_top(),
                Action::GotoBottom => self.go_bottom(),
                Action::EnterArticle => self.enter_article(),
                Action::GoBack => self.go_back(),
                Action::Search => self.search(),
                Action::Reset => self.reset(),
                // TODO: command mode (help, statusbar, etc.)
                _ => continue,
            }
            match self.mode {
                Mode::Select => {
                    let subset_titles = self.get_subset().to_owned();
                    let relative_selected_row = self.selected_row - self.row_offset;
                    self.renderer.print_titles(
                        &subset_titles,
                        relative_selected_row,
                        self.term_height,
                    );
                }
                Mode::Article => {
                    let subset_article = self.get_subset().to_owned();
                    self.renderer.print_article(&subset_article);
                }
            }
        }
        self.renderer.clear_all();
        self.renderer.show_cursor();
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

    fn go_top(&mut self) {
        self.row_offset = 0;
        if self.mode == Mode::Select {
            self.selected_row = 0;
        }
    }

    fn go_bottom(&mut self) {
        // TODO: handle Mode::Article
        self.selected_row = self.articles.len() - 1;
        self.row_offset = self.articles.len() - self.term_height;
    }

    fn enter_article(&mut self) {
        if !(self.mode == Mode::Select) {
            return;
        }
        self.mode = Mode::Article;

        let url = self.articles[self.selected_row].href.as_str();
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

        let subset_article = self.get_subset().to_owned();
        self.renderer.print_article(&subset_article);
    }

    fn go_back(&mut self) {
        if !(self.mode == Mode::Article) {
            return;
        }
        self.mode = Mode::Select;
        self.go_top();
    }

    fn search(&mut self) {
        // TODO: improve this
        self.reset();

        let mut search_string = String::new();
        loop {
            self.renderer
                .write_string(format!("{}{}", '/', search_string), self.term_height + 1);

            let keystroke = self.renderer.get_keystroke();

            match keystroke {
                Key::Esc => {
                    self.reset();
                    self.renderer.clear_status_bar(self.term_height);
                    break;
                }
                Key::Backspace if search_string.is_empty() => {
                    self.reset();
                    self.renderer.clear_status_bar(self.term_height);
                    break;
                }
                Key::Char('\n') => break,
                Key::Char(c) => search_string.push(c),
                Key::Backspace => {
                    search_string.pop();
                }
                _ => (),
            }

            self.reset();

            let re = Regex::new(search_string.as_str()).unwrap_or(Regex::new("").unwrap());

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

            let matches_titles = self.get_subset().to_owned();
            self.renderer
                .print_titles(&matches_titles, self.selected_row, self.term_height);
        }
    }

    fn reset(&mut self) {
        self.articles = self.all_articles.clone();
        self.titles = util::articles_to_titles(&self.articles)
            .into_iter()
            .take(self.articles.len())
            .collect::<Vec<String>>();
        self.go_top();
    }

    fn get_subset(&self) -> &[String] {
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
}
