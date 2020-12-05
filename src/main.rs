use rustyline::error::ReadlineError;
use rustyline::Editor;

mod commands;
mod utils;

fn verify_line(line: &str) {
    let mut params = line.split_whitespace();
    let command = params.next();
    match command.unwrap() {
        "clean" => commands::clean::run(),
        "close" => commands::close::run(0),
        // "exec" => commands::exec::run(params.next()),
        "" => utils::alerts::warning("You must enter a command"),
        _ => utils::alerts::error("Command not found"),
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(".catsh_history").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("-| ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                verify_line(&line);
            }
            Err(ReadlineError::Interrupted) => {
                utils::alerts::warning("If you want to exit use the 'close' command");
            }
            Err(ReadlineError::Eof) => {
                continue;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(".catsh_history").unwrap();
}
