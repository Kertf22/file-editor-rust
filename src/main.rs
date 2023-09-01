mod editor;
mod output;
mod reader;

use crossterm::terminal;
use editor::Editor;
use output::Output;
pub const VERSION: &str = "0.0.1";
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Output::clear_screen().expect("Error");
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;

    // let mut buf = [0; 1];
    terminal::enable_raw_mode()?;

    let mut editor = Editor::new();

    while editor.run()? {}

    Ok(())
}
