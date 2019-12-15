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
    // timestamp used for calculating how long it took to write one word
    pub timestamp: std::time::SystemTime,
}

/// struct representing statistics for current word
#[derive(Debug)]
pub struct Stat {
    /// time it took to complete this word
    pub time: std::time::Duration,
    /// how many mistakes were made during writing of this word
    pub mistakes: usize,

    pub word_len: usize,
}

impl State {
    pub fn from(text: Vec<String>) -> State {
        State { text,
                current_text_index: 0,
                current_word_index: 0,
                stats: vec![],
                current_errors: vec![],
                finished: false,
                timestamp: std::time::SystemTime::now() }
    }

    pub fn update(&mut self, input: crossterm::KeyEvent) {
        use crossterm::KeyEvent;
        use std::time::SystemTime;

        if self.finished {
            return;
        }

        // create instance of Stat object for current word, if not present
        if self.current_text_index == self.stats.len() {
            self.stats.push(Stat { time: std::time::Duration::new(0, 0),
                                   mistakes: 0,
                                   word_len: self.text[self.current_text_index].len() });
            self.timestamp = SystemTime::now()
        }

        match input {
            KeyEvent::Backspace => {
                if self.current_errors.is_empty() {
                    if self.current_word_index > 0 {
                        self.current_word_index -= 1;
                    }
                    return;
                }
                self.current_errors.pop();
            }
            KeyEvent::Char(key) => {
                if key == '\n' {
                    return;
                }
                if !self.current_errors.is_empty() {
                    // if there are any errors already stacked up, add this one too and return
                    self.current_errors.push(key);
                    self.stats[self.current_text_index].mistakes += 1
                } else if self.current_word_index == self.text[self.current_text_index].len() {
                    // space after word
                    if key == ' ' {
                        // finished word
                        self.stats[self.current_text_index].time =
                            self.timestamp.elapsed().expect("couldn't calculate time");
                        self.current_word_index = 0;
                        self.current_text_index += 1;
                    } else {
                        self.current_errors.push(key);
                        self.stats[self.current_text_index].mistakes += 1
                    }
                } else {
                    let curr_letter =
                        self.text[self.current_text_index].clone()
                                                          .chars()
                                                          .nth(self.current_word_index)
                                                          .expect("failed getting current index");
                    // word
                    if key == curr_letter {
                        self.current_word_index += 1;
                    } else {
                        self.current_errors.push(key);
                        self.stats[self.current_text_index].mistakes += 1
                    }
                    if self.current_text_index == self.text.len() - 1
                       && self.current_word_index == self.text.last().unwrap().len()
                    {
                        // finished everything
                        self.stats[self.current_text_index].time =
                            self.timestamp.elapsed().expect("couldn't calculate time");
                        self.finished = true;
                    }
                }
            }
            _ => {}
        }
    }
}
