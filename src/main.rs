mod app;
mod args;
mod input;
mod renderer;
mod scrape;
mod state;
mod util;

extern crate termion;

use crate::app::App;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = None;
    let cli = args::Args::parse();

    if !cli.category.is_empty() {
        url = Some(format!("https://nos.nl/nieuws/{}", cli.category))
    }

    let mut app = App::new(url);

    if cli.random {
        app.enter_random_article();
    }

    app.main();

    Ok(())
}
