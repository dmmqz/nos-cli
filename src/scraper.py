import requests
from bs4 import BeautifulSoup


class Scraper:
    """docstring for Scraper."""

    def get_soup(self, url: str):
        res = requests.get(url)
        res.raise_for_status()

        return BeautifulSoup(res.text, "html.parser")

    def get_items(self) -> tuple[list[str], list[str]]:
        """Get headlines from NOS.nl using webscraping."""
        soup = self.get_soup("https://nos.nl/sport/laatste")
        articles = soup.select("section > ul > li")

        titles = []
        links = []
        for article in articles:
            title = article.find("h2").text
            relative_date = article.find("time").text
            link = article.find("a")["href"]

            full_title = f"{title} ({relative_date})"

            titles.append(full_title)
            links.append(f"https://nos.nl{link}")

        return titles, links

    def get_article(self, url: str) -> list[str]:
        """Get the text from an article."""
        soup = self.get_soup(url)

        divs = soup.select("main > div")

        text = []
        for div in divs:
            for tag in div.find_all(["p", "h2"], recursive=False):  # TODO: make h2 bold
                text.append(tag.text)

        return text


if __name__ == "__main__":
    scraper = Scraper()
    scraper.get_article(
        "https://nos.nl/artikel/2569090-nederlandse-turners-snel-uitgeschakeld-bij-ek-in-nieuwe-gemengde-landenwedstrijd"
    )
