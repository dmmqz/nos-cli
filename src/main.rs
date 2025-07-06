use reqwest;
use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let link = "https://nos.nl/sport/laatste";

    let body = reqwest::blocking::get(link)?.text()?;
    let document = Html::parse_document(&body);

    let article_selector = Selector::parse("section > ul > li").unwrap();
    let title_selector = Selector::parse("h2").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let datetime_selector = Selector::parse("span > time").unwrap();

    for article in document.select(&article_selector) {
        if let Some(title_element) = article.select(&title_selector).next() {
            let title = title_element.text().collect::<Vec<_>>().join(" ");
            println!("{}", title)
        }

        if let Some(link_element) = article.select(&link_selector).next() {
            if let Some(href) = link_element.value().attr("href") {
                println!("https://nos.nl{}", href)
            }
        }

        // TODO: get datetime
        if let Some(datetime_element) = article.select(&datetime_selector).next() {
            let datetime = datetime_element.text().collect::<Vec<_>>().join(" ");
            println!("{}", datetime)
        }
    }
    Ok(())
}
