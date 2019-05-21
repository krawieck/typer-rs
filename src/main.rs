extern crate structopt;
extern crate termion;

mod args_parser;

use args_parser::Args;
fn main() {
    use structopt::StructOpt;

    let args = Args::from_args();
    let text = get_text(&args);
    println!("{:?}", text);
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
