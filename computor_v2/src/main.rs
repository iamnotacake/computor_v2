extern crate computor_v2;
extern crate rustyline;

use computor_v2::Context;
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut context = Context::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => match computor_v2::parse(&line) {
                Ok(expr) => {
                    println!("Parsed: {}", expr.to_string());
                    println!("Result: {}", expr.run(&mut context).to_string())
                }
                Err(err) => println!("Error: {}", err),
            },
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => break,
            Err(err) => panic!("{}", err),
        }
    }
}
