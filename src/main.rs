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


    for s in stdin.keys() {
        let (width, height) = terminal_size().expect("couldn't get terminal size");
        // check if user wants to quit
        match s {
            Ok(Key::Ctrl('c')) => break,
            _ => {}
        }
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .expect("oopsie poopsie i failed, plzzzzz senpai don't be engwy (*^_^*)");

        println!("{:?} size = ({}, {})", s, width, height);
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
