mod input;
mod scrape;
mod state;
mod util;

extern crate termion;

use crate::state::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.main();
    Ok(())
}
