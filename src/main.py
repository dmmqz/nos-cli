import os

import cursor
import requests
from bs4 import BeautifulSoup
from pynput import keyboard

import constants


class Interface:
    """docstring for Interface."""

    def __init__(self):
        self.selected_row = 0
        self.stop = False

        self.titles = self.get_items()

    def get_items(self) -> list[str]:
        """Get headlines from NOS.nl using webscraping."""
        res = requests.get("https://nos.nl/sport/laatste")
        # res = requests.get("https://nos.nl/nieuws/tech")
        res.raise_for_status()

        soup = BeautifulSoup(res.text, "html.parser")

        articles = soup.select("section > ul > li")

        titles = []
        for article in articles:
            title = article.find("h2").text
            relative_date = article.find("time").text

            full_title = f"{title} ({relative_date})"

            titles.append(full_title)

        return titles

    def print_items(self, n: int = 10):
        """Show headlines in the terminal."""
        os.system("clear")  # TODO: make windows compatible

        for i, title in enumerate(self.titles[:n]):
            if i == self.selected_row:
                print(f"{constants.WHITE_BACKGROUND}{constants.BLACK_TEXT}{title}{constants.RESET}")
            else:
                print(title)

    def handle_key(self, key: str) -> None:
        """Handle key inputs."""
        # TODO: stop getting all keys pressed once program has exited
        try:
            if key.char == "q":
                print("Quitting...")
                self.stop = True
            if key.char == "j" and self.selected_row < len(self.titles) - 1:
                self.selected_row += 1
            if key.char == "k" and self.selected_row > 0:
                self.selected_row -= 1
            if key.char == "b":  # TODO: return to previous page
                pass
        except AttributeError:
            # Special keys
            if key == keyboard.Key.enter:  # TODO: read article
                pass

        self.update_screen()

    def update_screen(self) -> None:
        """Update the screen."""
        os.system("clear")
        self.print_items()

    def main(self):
        """Main loop function."""
        cursor.hide()

        self.print_items()

        with keyboard.Listener(on_press=self.handle_key) as listener:
            while not self.stop:
                pass

        cursor.show()


if __name__ == "__main__":
    cli = Interface()
    cli.main()
