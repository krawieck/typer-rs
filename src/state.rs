/// struct representing current state of the program
pub struct State {
    /// vector containing all the text
    text: Vec<String>,
    current_text_index: usize,
    current_word_index: usize,
    stats: Vec<Stat>,
    current_errors: Vec<char>,
}

/// struct representing statistics for current word
pub struct Stat {
    /// time it took to complete this word
    time: f64,
    /// how many mistakes were made during writing of this word
    mistakes: usize,

    word_len: usize,
}

impl State {
    fn from(text: Vec<String>) -> State {
        return State {
            text: text,
            current_text_index: 0,
            current_word_index: 0,
            stats: vec![],
            current_errors: vec![],
        };
    }
}
