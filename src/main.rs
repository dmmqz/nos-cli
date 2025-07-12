mod input;
mod scrape;
mod state;
mod util;

extern crate termion;

use input::Action;

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

    let articles = scrape::get_items()?;
    let max_items = articles.len();
    let titles = &util::articles_to_titles(articles.clone())[..max_items];

    let mut selected_row = 0;
    let mut row_offset = 0;
    let mut mode = Mode::Select;

    let _ = write!(stdout, "{}", cursor::Hide);

    // Main TUI loop
    let mut bytes = stdin.bytes();
    print_titles(&mut stdout, titles, selected_row);
    loop {
        let b = bytes.next().unwrap().unwrap();
        let action = input::handle_input(b);

        match action {
            Action::Quit => break,
            // In the future:
            // Action::MoveUp => app_state.move_up();
            Action::MoveUp if selected_row > 0 => {
                selected_row -= 1;
                if selected_row + 1 == row_offset {
                    row_offset -= 1;
                }
            }
            Action::MoveDown if selected_row + 1 < max_items => {
                selected_row += 1;
                if selected_row - row_offset + 1 > term_height {
                    row_offset += 1;
                }
            }
            Action::GotoTop => {
                selected_row = 0;
                row_offset = 0;
            }
            Action::GotoBottom => {
                selected_row = max_items - 1;
                row_offset = max_items - term_height;
            }
            Action::EnterArticle if mode == Mode::Select => {
                mode = Mode::Article;
                print_article(
                    &mut stdout,
                    articles[selected_row].clone().href,
                    titles[selected_row].clone(),
                );
                selected_row = 0;
            }
            Action::GoBack if mode == Mode::Article => {
                mode = Mode::Select;
                selected_row = 0;
            }
            // TODO: Search
            // Action::Search if mode == Mode::Select =>
            _ => continue,
        }
        if mode == Mode::Select {
            let start_idx = row_offset;
            let end_idx = std::cmp::min(start_idx + term_height, max_items);
            let subset_titles = &titles[start_idx..end_idx];
            print_titles(&mut stdout, subset_titles, selected_row - row_offset);
        }

        stdout.flush()?;
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
