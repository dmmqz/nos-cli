use crate::scrape;

pub fn element_to_text(element: scraper::ElementRef) -> String {
    return element.text().collect::<Vec<_>>().join("");
}

pub fn article_to_title(article: scrape::Article) -> String {
    return format!("{} ({})", article.title, article.datetime);
}
