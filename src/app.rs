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
}

impl App {
    pub fn new(url: Option<String>) -> Self {
        let link = url.unwrap_or(String::from("https://nos.nl/nieuws/laatste"));
        let articles = scrape::get_items(link).expect("Couldn't get article titles.");

        let mut renderer = Renderer::new();
        let state = State::new(articles);

        renderer.hide_cursor();

        App { renderer, state }
    }

    pub fn main(&mut self) {
        let subset_titles = self.state.get_subset().to_owned();

        if self.state.mode == Mode::Select {
            self.renderer
                .print_titles(&subset_titles, self.state.get_relative_row());
        }

        loop {
            let keystroke = self.renderer.get_keystroke();
            let action = input::handle_input(keystroke);

            match action {
                Action::Quit => break,
                Action::MoveUp => self.state.move_up(),
                Action::MoveDown => self.state.move_down(),
                Action::GotoTop => self.state.go_top(),
                Action::GotoBottom => self.state.go_bottom(),
                Action::EnterArticle => self.enter_article(),
                Action::GoBack => self.state.go_back(),
                Action::Search => self.search(),
                Action::Reset => self.state.reset(),
                // TODO: command mode (help, statusbar, etc.)
                // TODO: command/search history, arrows to go through it
                Action::CommandMode => self.command_mode(),
                // TODO: center screen (vim zz)
                _ => continue,
            }
            match self.state.mode {
                Mode::Select => {
                    let subset_titles = self.state.get_subset().to_owned();
                    let relative_selected_row = self.state.get_relative_row();
                    self.renderer
                        .print_titles(&subset_titles, relative_selected_row);
                }
                Mode::Article => {
                    let subset_article = self.state.get_subset().to_owned();
                    self.renderer.print_article(&subset_article);
                }
            }
        }
        self.renderer.clear_all();
        self.renderer.show_cursor();
    }

    fn enter_article(&mut self) {
        self.state.enter_article();

        let subset_article = self.state.get_subset().to_owned();
        self.renderer.print_article(&subset_article);
    }

    fn input_mode<F, G>(&mut self, starting_char: char, on_submit: F, on_update: Option<G>)
    where
        F: FnOnce(&mut Self, &str),
        G: Fn(&mut Self, &str),
    {
        let mut input_string = String::new();
        loop {
            self.renderer
                .write_string(format!("{}{}", starting_char, input_string));

            let keystroke = self.renderer.get_keystroke();

            match keystroke {
                Key::Esc => {
                    self.state.reset();
                    self.renderer.clear_status_bar();
                    break;
                }
                Key::Backspace if input_string.is_empty() => {
                    self.renderer.clear_status_bar();
                    break;
                }
                Key::Char('\n') => {
                    on_submit(self, &input_string);
                    break;
                }
                Key::Char(c) => input_string.push(c),
                Key::Backspace => {
                    input_string.pop();
                }
                _ => (),
            }
            if let Some(ref update_fn) = on_update {
                update_fn(self, &input_string);
            }
        }
    }

    fn search(&mut self) {
        if self.state.mode == Mode::Article {
            return;
        }

        self.state.reset();

        self.input_mode(
            '/',
            |_, _| {}, // TODO: also make this an optional parameter
            Some(|this: &mut Self, input: &str| {
                let matches_titles = this.state.filter_articles(input);
                this.renderer.print_titles(&matches_titles, 0);
            }),
        );
    }

    fn command_mode(&mut self) {
        self.input_mode(
            ':',
            |this: &mut Self, input: &str| {
                this.execute_command(input.to_string());
            },
            None::<fn(&mut Self, &str)>,
        );
    }

    fn execute_command(&mut self, command: String) {
        match command.as_str() {
            "random" => self.enter_random_article(),
            "reset" | "noh" => self.state.reset(),
            // TODO: switch category
            s => self
                .renderer
                .write_error_string(format!("{} is not a valid command!", s)),
        }
    }

    pub fn enter_random_article(&mut self) {
        self.state.random_article();
        let subset_article = self.state.get_subset().to_owned();
        self.renderer.print_article(&subset_article);
    }
}
