use termion::event::Key;

use crate::{
    input::{self, Action},
    renderer::Renderer,
    scrape,
    state::{Mode, State},
};

pub struct App {
    renderer: Renderer<'static>,
    state: State,
    term_height: usize,
    term_width: usize,
}

impl App {
    pub fn new(url: Option<String>) -> Self {
        let (term_width, term_height) = termion::terminal_size().unwrap();
        let link = url.unwrap_or(String::from("https://nos.nl/nieuws/laatste"));
        let articles = scrape::get_items(link).expect("Couldn't get article titles.");

        let mut renderer = Renderer::new();
        let state = State::new(articles);
        let term_width = term_width as usize;
        let term_height = term_height as usize - 1;

        renderer.hide_cursor();

        App {
            renderer,
            state,
            term_height,
            term_width,
        }
    }

    pub fn main(&mut self) {
        let subset_titles = self.state.get_subset(self.term_height).to_owned();
        if self.state.mode == Mode::Select {
            self.renderer.print_titles(
                &subset_titles,
                self.state.get_relative_row(),
                self.term_height,
            );
        }
        loop {
            let keystroke = self.renderer.get_keystroke();
            let action = input::handle_input(keystroke);

            match action {
                Action::Quit => break,
                Action::MoveUp => self.state.move_up(),
                Action::MoveDown => self.state.move_down(self.term_height),
                Action::GotoTop => self.state.go_top(),
                Action::GotoBottom => self.state.go_bottom(self.term_height),
                Action::EnterArticle => self.enter_article(),
                Action::GoBack => self.state.go_back(),
                Action::Search => self.search(),
                Action::Reset => self.state.reset(),
                // TODO: command mode (help, statusbar, etc.)
                // TODO: center screen (vim zz)
                _ => continue,
            }
            match self.state.mode {
                Mode::Select => {
                    let subset_titles = self.state.get_subset(self.term_height).to_owned();
                    let relative_selected_row = self.state.get_relative_row();
                    self.renderer.print_titles(
                        &subset_titles,
                        relative_selected_row,
                        self.term_height,
                    );
                }
                Mode::Article => {
                    let subset_article = self.state.get_subset(self.term_height).to_owned();
                    self.renderer.print_article(&subset_article);
                }
            }
        }
        self.renderer.clear_all();
        self.renderer.show_cursor();
    }

    fn enter_article(&mut self) {
        if self.state.enter_article(self.term_width).is_err() {
            return;
        }

        let subset_article = self.state.get_subset(self.term_height).to_owned();
        self.renderer.print_article(&subset_article);
    }

    fn search(&mut self) {
        // TODO: improve this function
        if self.state.mode == Mode::Article {
            return;
        }
        self.state.reset();

        let mut search_string = String::new();
        loop {
            self.renderer
                .write_string(format!("{}{}", '/', search_string), self.term_height + 1);

            let keystroke = self.renderer.get_keystroke();

            match keystroke {
                Key::Esc => {
                    self.state.reset();
                    self.renderer.clear_status_bar(self.term_height);
                    break;
                }
                Key::Backspace if search_string.is_empty() => {
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

            let matches_titles = self
                .state
                .filter_articles(self.term_height, search_string.as_str());

            self.renderer
                .print_titles(&matches_titles, 0, self.term_height);
        }
    }

    pub fn random_article(&mut self) {
        if self.state.random_article(self.term_width).is_err() {
            return;
        }

        let subset_article = self.state.get_subset(self.term_height).to_owned();
        self.renderer.print_article(&subset_article);
    }
}
