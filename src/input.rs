use termion::event::Key;

// Handle key inputs
pub enum Action {
    Quit,
    GoBack,
    MoveUp,
    MoveDown,
    GotoTop,
    GotoBottom,
    EnterArticle,
    Search,
    Reset,
    None,
}

pub fn handle_input(key: Key) -> Action {
    match key {
        Key::Char('q') | Key::Esc => Action::Quit,
        Key::Char('k') | Key::Up => Action::MoveUp,
        Key::Char('j') | Key::Down => Action::MoveDown,
        Key::Char('g') => Action::GotoTop,
        Key::Char('G') => Action::GotoBottom,
        Key::Char('b') => Action::GoBack,
        Key::Char('i') => Action::EnterArticle,
        Key::Char('/') => Action::Search,
        Key::Char('r') => Action::Reset,
        _ => Action::None,
    }
}
