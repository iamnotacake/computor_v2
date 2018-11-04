extern crate computor_v2;
extern crate rustyline;

use rustyline::error::ReadlineError;

fn process_line(line: &str) {
    // TODO
}

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => process_line(&line),
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => break,
            Err(err) => panic!("{}", err),
        }
    }
}
