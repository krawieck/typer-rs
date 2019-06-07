extern crate crossterm;
extern crate structopt;

mod args_parser;
mod state;

use args_parser::Args;
use state::State;

fn main() -> std::io::Result<()> {
    use crossterm::{AlternateScreen, Crossterm, RawScreen};
    use std::io::{stdout, Write};
    use structopt::StructOpt;

    let alt = AlternateScreen::to_alternate(true)?;
    let input = crossterm::input();

    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();

    let args = Args::from_args();
    let text = get_text(&args);
    println!("{:?}", text);

    let mut state = State::from(text);
    let mut stdin = input.read_sync();
    loop {
        // for key in input.read_async() {
        use crossterm::{InputEvent, KeyEvent};

        let key = stdin.next();
        // check if user wants to quit
        let key = match key {
            Some(k) => k,
            _ => continue,
        };
        let key = match key {
            InputEvent::Keyboard(k) => k,
            _ => continue,
        };
        match key {
            KeyEvent::Ctrl('c') => {
                break;
            }
            _ => {}
        }

        state.update(key);
        render(&state, &terminal, &cursor)?;

        std::io::stdout().flush()?;
        // screen.flush().unwrap();
    }
    alt.to_main()?;
    Ok(())
}

fn get_text(args: &Args) -> Vec<String> {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    if args.take_from_stdin {
        let mut s: String = String::new();
        for line in stdin.lock().lines() {
            s.push(' ');
            s.push_str(line.unwrap_or("".to_string()).as_str());
        }
        s.split_whitespace().map(|a| a.to_string()).collect()
    } else {
        // @TODO HERE IT SHOULD GET RANDOM TEXT FROM SOMEWHERE
        "That is a text that you get when you don't take from stdin!
        It should be removed by the time anyone sees this program"
            .split_whitespace()
            .map(|a| a.to_string())
            .collect()
    }
}

fn render(
    state: &State,
    terminal: &crossterm::Terminal,
    cursor: &crossterm::TerminalCursor,
) -> std::io::Result<()> {
    use crossterm::{style, ClearType, Color, Colored};

    // place cursor in starting position
    terminal.clear(ClearType::All)?;
    cursor.goto(0, 0)?;

    if state.finished {
        terminal.clear(ClearType::All)?;
        terminal.write("you won!")?;
        return Ok(());
    }

    // Print text before current word
    if state.current_text_index > 0 {
        for s in &state.text[0..state.current_text_index] {
            print!("{} ", s);
        }
    }

    // Print text already written in current sentence
    if state.current_word_index > 0 {
        print!(
            "{}{}",
            Colored::Bg(Color::Green),
            &state.text[state.current_text_index][0..state.current_word_index],
        );
    }
    cursor.save_position()?;
    // Reset the color and print text not yet written in current sentence
    print!(
        "{}{}",
        style("").on(Color::Reset),
        &state.text[state.current_text_index][state.current_word_index..],
    );

    print!(" ");

    // Print text after current sentence
    if state.current_text_index != state.text.len() - 1 {
        for s in &state.text[state.current_text_index + 1..] {
            print!("{} ", s);
        }
    }

    // print errors
    cursor.reset_position()?;
    print!(
        "{}{}{}",
        Colored::Bg(Color::Red),
        state.current_errors.iter().collect::<String>(),
        style("").on(Color::Reset),
    );
    Ok(())
}
