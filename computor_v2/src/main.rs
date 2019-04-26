extern crate color_backtrace;
extern crate computor_v2;
extern crate failure;
extern crate rustyline;

use computor_v2::Context;
use failure::Fail;
use rustyline::error::ReadlineError;

fn main() {
    color_backtrace::install();
    let mut rl = rustyline::Editor::<()>::new();
    let mut context = Context::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => match computor_v2::parse(&line) {
                Ok(expr) => {
                    println!("Parsed: {}", &expr);

                    match expr.run(&mut context) {
                        Ok(result) => println!("Result: {}", &result),
                        Err(err) => println!("Error: {}", err),
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
