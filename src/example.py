import requests
from bs4 import BeautifulSoup
from datetime import datetime

res = requests.get("https://nos.nl/sport/laatste")
res.raise_for_status()

soup = BeautifulSoup(res.text, "html.parser")

articles = soup.select("section > ul > li")

for article in articles[:1]:
    title = article.find("h2").text
    print(title)

    link = article.find("a")["href"]
    print(f"https://nos.nl{link}")

    date = datetime.fromisoformat(article.find("time")["datetime"])
    print(article.find("time").text)
    # print(date)
