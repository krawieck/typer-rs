use crate::state::State;

pub fn render_game(
    state: &State,
    terminal: &crossterm::Terminal,
    cursor: &crossterm::TerminalCursor,
) -> std::io::Result<()> {
    use crossterm::{style, ClearType, Color, Colored};

    // place cursor in starting position
    terminal.clear(ClearType::All)?;
    cursor.goto(0, 0)?;

    if state.finished {
        // terminal.clear(ClearType::All)?;
        // terminal.write("you won!")?;
        // print!("{:?}", state);
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

pub fn render_stats(state: &State) {
    print!("{:?}", state)
}
