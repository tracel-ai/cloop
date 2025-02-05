use clap::Subcommand;
use cloop::{ClapSubcommandParser, Shell, ShellAction, ShellResult};

#[derive(Subcommand)]
pub enum Command {
    Echo { message: String },
    Add { a: i32, b: i32 },
    Prompt { message: String },
    Exit,
}

fn main() {
    let app_name = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "Demo".to_string());
    let delim = ">> ";

    let handler = |args: Command, _: &mut ()| -> ShellResult {
        match args {
            Command::Echo { message } => {
                println!("{}", message);
                Ok(ShellAction::Continue)
            }
            Command::Add { a, b } => {
                println!("{}", a + b);
                Ok(ShellAction::Continue)
            }
            Command::Prompt { message } => {
                Ok(ShellAction::UpdatePrompt(format!("{message}{delim}")))
            }
            Command::Exit => Ok(ShellAction::Exit),
        }
    };

    let mut shell = Shell::new(
        format!("{app_name}{delim}"),
        (),
        rustyline::DefaultEditor::new().unwrap(),
        ClapSubcommandParser::default(),
        handler,
    );

    shell.run().unwrap();
}
