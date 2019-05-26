extern crate structopt;
extern crate termion;

mod args_parser;
mod state;

use args_parser::Args;
use state::State;

fn main() {
    use std::io::{stdin, stdout, Write};
    use structopt::StructOpt;
    use termion::event::{Event, Key};
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;
    use termion::terminal_size;

    let args = Args::from_args();
    let text = get_text(&args);
    println!("{:?}", text);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut state = State::from(text);

    for s in stdin.keys() {
        let (width, height) = terminal_size().expect("couldn't get terminal size");
        // check if user wants to quit
        match s {
            Ok(Key::Ctrl('c')) => break,
            _ => {}
        }
        let s = s.expect("couldn't get key");
        update_state(&mut state, s.clone());

        // DEBUG CODE, SHOULD BE REMOVED BEFORE OFFICIAL RELEASE
        write!(
            stdout,
            "{}{}text: {:?} \nerrors: {:?}, {}text_index: {}, word_index: {}. finished: {}. {:?}  ",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            state.text,
            state.current_errors,
            termion::cursor::Goto(1, 5),
            state.current_text_index,
            state.current_word_index,
            state.finished,
            s,
        )
        .expect("oopsie poopsie i failed, plzzzzz senpai don't be engwy (*^_^*)");

        stdout.flush().unwrap();
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
        "This is a text that you get when you don't take from stdin!
        It should be removed by the time anyone sees this program"
            .split_whitespace()
            .map(|a| a.to_string())
            .collect()
    }
}

fn update_state(state: &mut State, input: termion::event::Key) {
    use termion::event::Key::{Backspace, Char};
    match input {
        Backspace => {
            if state.current_errors.is_empty() {
                if state.current_word_index > 0 {
                    state.current_word_index -= 1;
                }
                return;
            }
            state.current_errors.pop();
            return;
        }
        Char(key) => {
            if key == '\n' {
                return;
            }
            if !state.current_errors.is_empty() {
                // if there are any errors already stacked up, add this one too and return
                state.current_errors.push(key);
            } else if state.current_word_index == state.text[state.current_text_index].len() {
                // space after word
                if key == ' ' {
                    state.current_word_index = 0;
                    state.current_text_index += 1;
                } else {
                    state.current_errors.push(key);
                }
            } else {
                let curr_index = state.text[state.current_text_index]
                    .clone()
                    .chars()
                    .nth(state.current_word_index)
                    .expect("failed getting current index");
                // word
                if key == curr_index {
                    state.current_word_index += 1;
                } else {
                    state.current_errors.push(key);
                }
                if state.current_text_index == state.text.len() - 1
                    && state.current_word_index == state.text.last().unwrap().len()
                {
                    state.finished = true;
                }
            }
        }
        _ => return,
    };
}
