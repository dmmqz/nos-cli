mod input;
mod scrape;
mod state;
mod util;

extern crate termion;

use crate::state::AppState;

// #[derive(PartialEq)]
// enum Mode {
//     Select,
//     Article,
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init
    let app = AppState::new();
    app.main();
    Ok(())
}
