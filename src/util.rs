use crate::scrape;

pub fn element_to_text(element: scraper::ElementRef) -> String {
    return element.text().collect::<Vec<_>>().join("");
}

pub fn articles_to_titles(articles: &Vec<scrape::Article>) -> Vec<String> {
    let (term_width, _) = termion::terminal_size().unwrap();
    let term_width = term_width as usize;
    let mut titles = Vec::new();

    for article in articles {
        let mut clipped_title = article.title.clone();
        // -6 because 3 dots + the space and parentheses below
        clipped_title.truncate(term_width - article.datetime.len() - 6);
        if clipped_title.len() != article.title.len() {
            clipped_title.push_str("...");
        }

        titles.push(format!("{} ({})", clipped_title, article.datetime));
    }

    return titles;
}
