mod input;
mod scrape;
mod state;
mod util;

extern crate termion;

use crate::state::AppState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init
    let mut app = AppState::new();
    app.main();
    Ok(())
}
