// Handle key inputs
pub enum Action {
    Quit,
    GoBack,
    MoveUp,
    MoveDown,
    GotoTop,
    GotoBottom,
    EnterArticle,
    // Search,
    None,
}

pub fn handle_input(key: u8) -> Action {
    match key {
        b'q' => Action::Quit,
        b'k' => Action::MoveUp,
        b'j' => Action::MoveDown,
        b'g' => Action::GotoTop,
        b'G' => Action::GotoBottom,
        b'b' => Action::GoBack,
        b'i' => Action::EnterArticle,
        _ => Action::None,
    }
}
