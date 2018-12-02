extern crate computor_v2;
extern crate rustyline;

use computor_v2::process_line;
use computor_v2::Context;
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut context = Context::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => process_line(&line, &mut context),
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => break,
            Err(err) => panic!("{}", err),
        }
    }
}
