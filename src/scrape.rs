use crate::util;
use reqwest;
use scraper::{Html, Selector};

pub struct Article {
    pub title: String,
    pub href: String,
    pub datetime: String,
}

pub fn get_items() -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let link = "https://nos.nl/sport/laatste";

    let body = reqwest::blocking::get(link)?.text()?;
    let document = Html::parse_document(&body);

    let article_selector = Selector::parse("section > ul > li").unwrap();
    let title_selector = Selector::parse("h2").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let datetime_selector = Selector::parse("span > time").unwrap();

    let mut articles = Vec::new();

    for article in document.select(&article_selector) {
        let title = article
            .select(&title_selector)
            .next()
            .map(util::element_to_text)
            .unwrap_or_default();

        let href = article
            .select(&link_selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|href| format!("https://nos.nl{}", href))
            .unwrap_or_default();

        let datetime = article
            .select(&datetime_selector)
            .next()
            .map(util::element_to_text)
            .unwrap_or_default();

        articles.push(Article {
            title,
            href,
            datetime,
        });
    }
    Ok(articles)
}
