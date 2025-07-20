use crate::util;
use reqwest;
use scraper::{Html, Selector};

#[derive(Clone)]
pub struct Article {
    pub title: String,
    pub href: String,
    pub datetime: String,
}

pub fn get_items() -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let url = "https://nos.nl/nieuws/laatste";

    let body = reqwest::blocking::get(url)?.text()?;
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

pub fn get_article(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    let document = Html::parse_document(&body);

    let text_selector = Selector::parse("main > div > p, main > div > h2").unwrap();

    let mut all_text = Vec::new();

    for element in document.select(&text_selector) {
        let element_text = util::element_to_text(element);
        all_text.push(element_text);
    }

    Ok(all_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_article() {
        let url = "https://nos.nl/collectie/13995/artikel/2573968-thuisland-zwitserland-schakelt-ijsland-uit-op-ek-en-houdt-zicht-op-kwartfinales";
        let result = get_article(url);

        match result {
            Ok(all_text) => println!("Text: {}", all_text[0]),
            Err(e) => panic!("Failed to get article: {}", e),
        }
    }
}
