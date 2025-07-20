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
    max_items: usize,
    titles: Vec<String>,
    selected_row: usize,
    row_offset: usize,
    mode: Mode,
    current_article_text: Vec<String>,
}

impl App {
    // TODO: split this struct up, e.g. Render, AppState structs
    pub fn new() -> Self {
        let (term_width, term_height) = termion::terminal_size().unwrap();

        let renderer = Renderer::new();

        let term_width = term_width as usize;
        let term_height = term_height as usize;
        let articles = scrape::get_items().expect("Couldn't get article titles.");
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
            max_items,
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
        self.renderer
            .print_titles(&subset_titles, self.selected_row - self.row_offset);
        self.renderer.flush();
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
                // TODO: Search
                // Action::Search if mode == Mode::Select =>
                // TODO: command mode (help, statusbar, etc.)
                _ => continue,
            }
            match self.mode {
                Mode::Select => {
                    let subset_titles = self.get_subset().to_owned();
                    let relative_selected_row = self.selected_row - self.row_offset;
                    self.renderer
                        .print_titles(&subset_titles, relative_selected_row);
                }
                Mode::Article => {
                    let subset_article = self.get_subset().to_owned();
                    self.renderer.print_article(&subset_article);
                }
            }

            self.renderer.flush();
        }
        self.renderer.clear();
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
        // TODO: handle Mode::Article
        self.selected_row = self.max_items - 1;
        self.row_offset = self.max_items - self.term_height;
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

    fn get_subset(&self) -> &[String] {
        let start_idx = self.row_offset;

        match self.mode {
            Mode::Select => {
                let end_idx = std::cmp::min(start_idx + self.term_height, self.max_items);
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
