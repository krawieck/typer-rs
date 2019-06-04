/// struct representing current state of the program
#[derive(Debug)]
pub struct State {
    /// vector containing all the text
    pub text: Vec<String>,
    pub current_text_index: usize,
    pub current_word_index: usize,
    pub stats: Vec<Stat>,
    pub current_errors: Vec<char>,
    pub finished: bool,
}

/// struct representing statistics for current word
#[derive(Debug)]
pub struct Stat {
    /// time it took to complete this word
    pub time: f64,
    /// how many mistakes were made during writing of this word
    pub mistakes: usize,

    pub word_len: usize,
}

impl State {
    pub fn from(text: Vec<String>) -> State {
        return State {
            text: text,
            current_text_index: 0,
            current_word_index: 0,
            stats: vec![],
            current_errors: vec![],
            finished: false,
        };
    }

    pub fn update(&mut self, input: termion::event::Key) {
        use termion::event::Key::{Backspace, Char};
        if self.finished {
            return;
        }
        match input {
            Backspace => {
                if self.current_errors.is_empty() {
                    if self.current_word_index > 0 {
                        self.current_word_index -= 1;
                    }
                    return;
                }
                self.current_errors.pop();
                return;
            }
            Char(key) => {
                if key == '\n' {
                    return;
                }
                if !self.current_errors.is_empty() {
                    // if there are any errors already stacked up, add this one too and return
                    self.current_errors.push(key);
                } else if self.current_word_index == self.text[self.current_text_index].len() {
                    // space after word
                    if key == ' ' {
                        self.current_word_index = 0;
                        self.current_text_index += 1;
                    } else {
                        self.current_errors.push(key);
                    }
                } else {
                    let curr_index = self.text[self.current_text_index]
                        .clone()
                        .chars()
                        .nth(self.current_word_index)
                        .expect("failed getting current index");
                    // word
                    if key == curr_index {
                        self.current_word_index += 1;
                    } else {
                        self.current_errors.push(key);
                    }
                    if self.current_text_index == self.text.len() - 1
                        && self.current_word_index == self.text.last().unwrap().len()
                    {
                        self.finished = true;
                    }
                }
            }
            _ => return,
        }
    }
}
