import os

import cursor
from pynput import keyboard

import constants
from scraper import Scraper


class Interface:
    """docstring for Interface."""

    def __init__(self) -> None:
        """Initialize class."""
        self.scraper = Scraper()

        self.selected_row = 0
        self.stop = False
        self.mode = "select"

        self.titles, self.links = self.scraper.get_items()

    def print_items(self, n: int = 10) -> None:
        """Show headlines in the terminal."""
        os.system("clear")  # TODO: make windows compatible

        for i, title in enumerate(self.titles[:n]):
            if i == self.selected_row:
                print(f"{constants.WHITE_BACKGROUND}{constants.BLACK_TEXT}{title}{constants.RESET}")
            else:
                print(title)

    def print_article(self, url: str) -> None:
        """Show article."""
        os.system("clear")
        article = self.scraper.get_article(url)

        for text in article:
            print(text + "\n")

    def handle_key(self, key: str) -> None:
        """Handle key inputs."""
        # TODO: stop getting all keys pressed once program has exited
        # TODO: ignore inputs based on mode
        try:
            if key.char == "q":
                print("Quitting...")
                self.stop = True
            if key.char == "j" and self.selected_row < 10 - 1:  # TODO: Make 10 based on len(titles)
                self.selected_row += 1
            if key.char == "k" and self.selected_row > 0:
                self.selected_row -= 1
            if key.char == "b":
                self.mode = "select"
        except AttributeError:
            # Special keys
            if key == keyboard.Key.enter:
                self.mode = "article"
                self.print_article(self.links[self.selected_row])

        self.update_screen()

    def update_screen(self) -> None:
        """Update screen based on current mode."""
        if self.mode == "select":
            os.system("clear")
            self.print_items()
        elif self.mode == "article":
            pass

    def main(self) -> None:
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
