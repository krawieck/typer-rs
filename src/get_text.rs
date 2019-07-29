use crate::args_parser::Args;

pub fn get_text(args: &Args) -> Vec<String> {
    use std::io::{stdin, BufRead};
    let stdin = stdin();
    if args.take_from_stdin {
        let mut s: String = String::new();
        for line in stdin.lock().lines() {
            s.push(' ');
            s.push_str(line.unwrap_or("".to_string()).as_str());
        }
        drop(stdin);
        return s.split_whitespace().map(|a| a.to_string()).collect();
    }
    // @TODO HERE IT SHOULD GET RANDOM TEXT FROM SOMEWHERE
    "That is a text that you get when you don't take from stdin!
        It should be removed by the time anyone sees this program"
        .split_whitespace()
        .map(|a| a.to_string())
        .collect()
}
