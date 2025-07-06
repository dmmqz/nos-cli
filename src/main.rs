mod scrape;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let articles = scrape::get_items()?;

    for article in articles {
        println!(
            "{}\n{}\n{}\n",
            article.title, article.href, article.datetime
        );
    }

    Ok(())
}
