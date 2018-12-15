extern crate computor_v2;
extern crate failure;
extern crate rustyline;

use computor_v2::Context;
use failure::Fail;
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut context = Context::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => match computor_v2::parse(&line) {
                Ok(expr) => {
                    println!("Parsed: {}", expr.to_string());

                    match expr.run(&mut context) {
                        Ok(result) => println!("Result: {}", result.to_string()),
                        Err(err) => {
                            println!("Error: {}", err);

                            if let Some(bt) = err.cause().and_then(|cause| cause.backtrace()) {
                                println!("{}", bt);
                            }
                        }
                    }
                }
                Err(err) => println!("Error: {}", err),
            },
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => break,
            Err(err) => panic!("{}", err),
        }
    }
}
