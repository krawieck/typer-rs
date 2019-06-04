extern crate structopt;
extern crate termion;

mod args_parser;
mod state;

use args_parser::Args;
use state::State;

fn main() {
    use std::io::{stdin, stdout, Write};
    use structopt::StructOpt;
    use termion::event::Key;
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;

    let args = Args::from_args();
    let text = get_text(&args);
    println!("{:?}", text);

    let stdin = stdin();
    let mut screen = termion::screen::AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut state = State::from(text);

    for s in stdin.keys() {
        // check if user wants to quit
        match s {
            Ok(Key::Ctrl('c')) => {
                break;
            }
            _ => {}
        }
        let s = s.expect("couldn't get key");

        state.update(s);
        render(&state, &mut screen);

        screen.flush().unwrap();
    }
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

fn render(state: &State, stdout: &mut std::io::Stdout) {
    use std::io::Write;
    use termion::{color, cursor};


    // place cursor in starting position
    write!(stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1))
        .expect("couldn't render to terminal");

    if state.finished {
        write!(stdout, "{}", termion::clear::All).unwrap();
        println!("you won!");
        return;
    }

    write!(stdout, "{}", color::Bg(color::Green)).unwrap();
    // Print text before current word
    if state.current_text_index > 0 {
        for s in &state.text[0..state.current_text_index] {
            write!(stdout, "{} ", s).unwrap();
        }
    }

    // Print text already written in current sentence
    if state.current_word_index > 0 {
        write!(
            stdout,
            "{}",
            &state.text[state.current_text_index][0..state.current_word_index]
        )
        .unwrap();
    }

    let (cur_x, cur_y) = termion::cursor::DetectCursorPos::cursor_pos(stdout).unwrap();

    // Reset the color and print text not yet written in current sentence
    write!(
        stdout,
        "{}{}",
        color::Bg(color::Reset),
        &state.text[state.current_text_index][state.current_word_index..]
    )
    .unwrap();

    write!(stdout, " ").unwrap();

    // Print text after current sentence
    if state.current_text_index != state.text.len() - 1 {
        for s in &state.text[state.current_text_index + 1..] {
            write!(stdout, "{} ", s).unwrap();
        }
    }

    // print errors
    write!(
        stdout,
        "{}{}{}{}",
        termion::cursor::Goto(cur_x, cur_y),
        color::Bg(color::Red),
        state.current_errors.iter().collect::<String>(),
        color::Bg(color::Reset),
    )
    .unwrap();
}
