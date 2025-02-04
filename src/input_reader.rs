use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{self, Write};

pub enum InputResult {
    Input(String),
    Interrupted,
    Eof,
}

pub trait InputReader {
    fn read(&mut self, prompt: &str) -> io::Result<InputResult>;
}

impl<F> InputReader for F
where
    F: FnMut(&str) -> io::Result<InputResult>,
{
    fn read(&mut self, prompt: &str) -> io::Result<InputResult> {
        self(prompt)
    }
}

pub struct StdioInputReader;
impl InputReader for StdioInputReader {
    fn read(&mut self, prompt: &str) -> io::Result<InputResult> {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        Ok(InputResult::Input(input))
    }
}

impl<H, I> InputReader for Editor<H, I>
where
    H: rustyline::Helper,
    I: rustyline::history::History,
{
    fn read(&mut self, prompt: &str) -> io::Result<InputResult> {
        match self.readline(&prompt) {
            Ok(s) => {
                self.add_history_entry(&s)
                    .map_err(convert_rustyline_to_io)?;
                Ok(InputResult::Input(s))
            }
            Err(ReadlineError::Eof) => Ok(InputResult::Eof),
            Err(ReadlineError::Interrupted) => Ok(InputResult::Interrupted),
            Err(e) => Err(convert_rustyline_to_io(e)),
        }
    }
}

fn convert_rustyline_to_io(e: ReadlineError) -> io::Error {
    match e {
        ReadlineError::Io(e) => e,
        #[cfg(unix)]
        ReadlineError::Errno(e) => e.into(),
        ReadlineError::WindowResized => io::Error::new(io::ErrorKind::Other, e),
        e => io::Error::new(io::ErrorKind::Interrupted, e),
    }
}
