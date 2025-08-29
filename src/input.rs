use termion::event::Key;

pub enum Action {
    Quit,
    GoBack,
    MoveUp,
    MoveDown,
    GotoTop,
    GotoBottom,
    PageUp,
    PageDown,
    EnterArticle,
    Search,
    Reset,
    CommandMode,
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
        Key::Ctrl('f') | Key::PageDown => Action::PageDown,
        Key::Ctrl('b') | Key::PageUp => Action::PageUp,
        Key::Char('\n') | Key::Char('i') => Action::EnterArticle,
        Key::Char('/') => Action::Search,
        Key::Char('r') => Action::Reset,
        Key::Char(':') => Action::CommandMode,
        _ => Action::None,
    }
}
