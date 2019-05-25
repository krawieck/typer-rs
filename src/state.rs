/// struct representing current state of the program
pub struct State {
    /// vector containing all the text
    pub text: Vec<String>,
    pub current_text_index: usize,
    pub current_word_index: usize,
    pub stats: Vec<Stat>,
    pub current_errors: Vec<char>,
}

/// struct representing statistics for current word
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
        };
    }
}
