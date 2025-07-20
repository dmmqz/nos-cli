mod app;
mod input;
mod renderer;
mod scrape;
mod util;

extern crate termion;

use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.main();
    Ok(())
}
