extern crate crossterm;
extern crate structopt;

mod args_parser;
mod get_text;
mod render;
mod state;

use get_text::get_text;
use render::{render_game, render_stats};

fn main() -> std::io::Result<()> {
    use args_parser::Args;
    use crossterm::{AlternateScreen, Crossterm};
    use state::State;
    use std::io::Write;
    use structopt::StructOpt;

    let args = Args::from_args();
    let alt = AlternateScreen::to_alternate(true)?;
    let input = crossterm::input();

    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();

    let text = get_text(&args);

    let mut state = State::from(text);
    render_game(&state, &terminal, &cursor)?;
    let mut stdin = input.read_sync();
    loop {
        use crossterm::{InputEvent, KeyEvent};

        if state.finished {
            break;
        }

        let key = stdin.next();
        // check if user wants to quit
        let key = match key {
            Some(k) => k,
            _ => continue,
        };
        let key = match key {
            InputEvent::Keyboard(k) => k,
            _ => continue,
        };
        if let KeyEvent::Ctrl('c') = key {
            break;
        }

        if !state.finished {
            state.update(key);
            render_game(&state, &terminal, &cursor)?;
            std::io::stdout().flush()?;
        } else {
            render_stats(&state);
            std::io::stdout().flush()?;
        }
    }

    alt.to_main()?;
    Ok(())
}
