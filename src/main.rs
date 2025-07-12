mod scrape;
mod util;

extern crate termion;

use std::io::{Read, Write, stdin, stdout};
use termion::raw::IntoRawMode;
use termion::{color, cursor};

#[derive(PartialEq)]
enum Mode {
    Select,
    Article,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    let (_, term_height) = termion::terminal_size().unwrap();
    let term_height = term_height as usize;

    let _ = write!(stdout, "{}", cursor::Hide);

    // Titles
    let articles = scrape::get_items()?;
    let max_items = std::cmp::min(term_height, articles.len());
    let titles = &util::articles_to_titles(articles.clone())[..max_items];

    let mut selected_row = 0;
    let mut mode = Mode::Select;

    // Main TUI loop
    let mut bytes = stdin.bytes();
    print_titles(&mut stdout, titles, selected_row);
    loop {
        let b = bytes.next().unwrap().unwrap();

        match b {
            // Quit
            b'q' => break,
            // Go up
            b'k' if selected_row > 0 => selected_row -= 1,
            // Go down
            b'j' if selected_row + 1 < max_items => selected_row += 1,
            // Go to top
            b'g' => selected_row = 0,
            // Go to bottom
            b'G' => selected_row = max_items - 1,
            // Enter article
            b'i' if mode == Mode::Select => {
                mode = Mode::Article;
                print_article(
                    &mut stdout,
                    articles[selected_row].clone().href,
                    titles[selected_row].clone(),
                );
                selected_row = 0;
            }
            // Exit article
            b'b' if mode == Mode::Article => {
                mode = Mode::Select;
                selected_row = 0;
            }
            _ => continue,
        }
        if mode == Mode::Select {
            print_titles(&mut stdout, titles, selected_row);
        }

        stdout.flush().unwrap();
        write!(stdout, "{}", termion::clear::All)?;
    }

    // Cleanup
    let _ = write!(stdout, "{}", cursor::Show);
    Ok(())
}

fn print_titles<W: Write>(stdout: &mut W, titles: &[String], selected_row: usize) {
    let _ = write!(stdout, "{}", termion::clear::All);
    for (i, title) in titles.iter().enumerate() {
        if i == selected_row {
            write!(
                stdout,
                "{}{}{}{}{}{}",
                termion::cursor::Goto(1, i as u16 + 1),
                color::Bg(color::White),
                color::Fg(color::Black),
                title,
                color::Fg(color::Reset),
                color::Bg(color::Reset),
            )
            .unwrap();
        } else {
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, i as u16 + 1),
                title
            )
            .unwrap();
        }
    }
}

fn print_article<W: Write>(stdout: &mut W, url: String, title: String) {
    let _ = write!(stdout, "{}", termion::clear::All);

    let all_text = scrape::get_article(url).unwrap();
    let _ = write!(
        stdout,
        "\r\n{}{}{}",
        termion::style::Bold,
        title,
        termion::style::Reset
    );

    for text in all_text {
        let _ = write!(stdout, "\r\n{}\r\n", text);
    }
}
