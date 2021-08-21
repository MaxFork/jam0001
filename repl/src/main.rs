use interpreter::parse::Parser;
use rustyline::{error::ReadlineError, Editor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        std::fs::write("history.txt", "")?;
    }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let mut parser = Parser::new(&line);
                let ast = parser.parse()?;

                println!("{:#?}", ast);
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
