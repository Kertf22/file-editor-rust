use crossterm::event::{KeyCode, KeyEvent, self};

use crate::{output::Output, reader::Reader};

pub struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    pub fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            KeyEvent {
                code: val @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right| KeyCode::PageUp | KeyCode::PageDown| KeyCode::Home | KeyCode::End),
                modifiers: event::KeyModifiers::NONE,
            } => self.output.move_cursor(val),
            _ => {}
        }

        Ok(true)
    }

    pub fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}
