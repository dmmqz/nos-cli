mod scrape;
mod util;

extern crate termion;

use std::io::{Read, Write, stdin, stdout};
use termion::raw::IntoRawMode;
use termion::{color, cursor};

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
    let titles = &util::articles_to_titles(articles)[..=term_height];

    let mut selected_row = 1;

    // Main TUI loop
    let mut bytes = stdin.bytes();
    print_titles(&mut stdout, titles, selected_row);
    loop {
        let b = bytes.next().unwrap().unwrap();

        match b {
            // Quit
            b'q' => break,
            // Go up
            b'k' if selected_row > 1 => selected_row -= 1,
            // Go down
            b'j' if selected_row < term_height => selected_row += 1,
            b'g' => selected_row = 1,
            b'G' => selected_row = term_height,
            _a => continue,
        }
        print_titles(&mut stdout, titles, selected_row);

        stdout.flush().unwrap();
        write!(stdout, "{}", termion::clear::All)?;
    }

    // Cleanup
    let _ = write!(stdout, "{}", cursor::Show);
    Ok(())
}

fn print_titles<W: Write>(stdout: &mut W, titles: &[String], selected_row: usize) {
    for (i, title) in titles.iter().enumerate() {
        if i == selected_row {
            write!(
                stdout,
                "\r\n{}{}{}{}{}",
                color::Bg(color::White),
                color::Fg(color::Black),
                // termion::cursor::Goto(1, i as u16),
                title,
                color::Fg(color::Reset),
                color::Bg(color::Reset),
            )
            .unwrap();
        } else {
            write!(stdout, "\r\n{}", title).unwrap();
        }
    }
}
