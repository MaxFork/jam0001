use interpreter::parse::Parser;
use rustyline::validate::{
    MatchingBracketValidator, ValidationContext, ValidationResult, Validator,
};
use rustyline::{error::ReadlineError, Editor};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};

#[derive(Completer, Helper, Highlighter, Hinter)]
struct InputValidator {
    brackets: MatchingBracketValidator,
}

impl Validator for InputValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        self.brackets.validate(ctx)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let h = InputValidator {
        brackets: MatchingBracketValidator::new(),
    };

    let mut rl = Editor::new();
    rl.set_helper(Some(h));
    if rl.load_history("history.txt").is_err() {
        std::fs::write("history.txt", "")?;
    }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if !line.trim().is_empty() {
                    rl.add_history_entry(line.as_str());

                    let mut parser = Parser::new(&line);
                    let ast = parser.parse()?;

                    println!("{:#?}", ast);
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(error) => {
                eprintln!("{}", error);
                break;
            }
        }
    }

    rl.save_history("history.txt")?;
    Ok(())
}
