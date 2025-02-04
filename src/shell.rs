use crate::input_reader::{InputReader, InputResult};
use crate::input_parser::InputParser;

pub type ShellResult = anyhow::Result<ShellAction>;

pub enum ShellAction {
    Continue,
    UpdatePrompt(String),
    Exit,
}

pub trait Handler<A, T> {
    fn handle(&self, args: A, context: &mut T) -> ShellResult;
}

/// A handler that takes a parser and a context and returns a ShellResult
/// This is a convenience implementation for closures
/// It allows you to pass a closure that takes a parser and a context and returns a ShellResult
/// as a handler to the Shell struct
impl<A, T, F> Handler<A, T> for F
where
    F: Fn(A, &mut T) -> ShellResult,
{
    fn handle(&self, args: A, context: &mut T) -> ShellResult {
        self(args, context)
    }
}

pub struct Shell<A, T, I: InputReader, P: InputParser<A, T>, H: Handler<A, T>> {
    prompt: String,
    context: T,
    input_reader: I,
    input_parser: P,
    handler: H,
    args_marker: std::marker::PhantomData<A>,
}

impl<A, T, I: InputReader, P: InputParser<A, T>, H: Handler<A, T>> Shell<A, T, I, P, H> {
    pub fn new(
        prompt: impl std::fmt::Display,
        context: T,
        input_reader: I,
        input_parser: P,
        handler: H,
    ) -> Self {
        Self {
            prompt: prompt.to_string(),
            context,
            input_reader,
            input_parser,
            handler,
            args_marker: std::marker::PhantomData,
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let input_res = self.input_reader.read(&self.prompt)?;
            match input_res {
                InputResult::Input(input) => {
                    let res = self.handle_input(&input);
                    match res {
                        Ok(shell_res) => match shell_res {
                            Ok(ShellAction::Continue) => continue,
                            Ok(ShellAction::UpdatePrompt(prompt)) => {
                                self.prompt = prompt;
                                continue;
                            }
                            Ok(ShellAction::Exit) => break,
                            Err(e) => println!("{}", e),
                        },
                        Err(e) => println!("{}", e),
                    }
                }
                InputResult::Interrupted => continue,
                InputResult::Eof => break,
            }
        }
        Ok(())
    }

    fn handle_input(&mut self, input: &str) -> Result<ShellResult, String> {
        let raw_args = shlex::split(input).ok_or("Invalid quoting")?;

        // Here we are using the parser to parse the arguments before handing them to the handler
        // so that in case of a parsing error, we can early return and print the error message
        let args = self
            .input_parser
            .parse(raw_args, &mut self.context)
            .map_err(|e| e.to_string())?;

        let res = self.handler.handle(args, &mut self.context);

        Ok(res)
    }
}
