struct AppState {
    mode: Mode,
    selected_row: usize,
    row_offset: usize,
    articles: Vec<Article>,
}

impl AppState {
    fn new(articles: Vec<Article>) -> Self {
        AppState {
            mode: Mode::Select,
            selected_row: 0,
            row_offset: 0,
            articles,
        }
    }

    fn move_up(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected_row < self.articles.len() - 1 {
            self.selected_row += 1;
        }
    }
}
