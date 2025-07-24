use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Category to show articles for
    #[arg(short, long, default_value_t = String::from("laatste"))]
    pub category: String,
}
