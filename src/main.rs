mod app;
mod args;
mod input;
mod renderer;
mod scrape;
mod util;

extern crate termion;

use crate::app::App;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    args::Args::parse();

    let mut app = App::new();
    app.main();
    Ok(())
}
