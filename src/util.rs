use crate::scrape;

pub fn element_to_text(element: scraper::ElementRef) -> String {
    return element.text().collect::<Vec<_>>().join("");
}

pub fn articles_to_titles(articles: Vec<scrape::Article>) -> Vec<String> {
    let mut titles = Vec::new();

    for article in articles {
        titles.push(format!("{} ({})", article.title, article.datetime));
    }

    return titles;
}
