extern crate color_backtrace;
extern crate computor_v2;
extern crate failure;
extern crate rustyline;

use computor_v2::Context;
use rustyline::error::ReadlineError;

fn main() {
    color_backtrace::install();
    let mut rl = rustyline::Editor::<()>::new();
    let mut context = Context::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                if line.starts_with("root ") {
                    computor_v2::computor_v1::computor_v1(line[5..].to_string());
                } else {
                    match computor_v2::parse(&line) {
                        Ok(expr) => {
                            println!("Parsed: {}", &expr);

                            match expr.run(&mut context, 0) {
                                Ok(result) => println!("Result: {}", &result),
                                Err(err) => println!("Error: {}", err),
                            }
                        }
                        Err(err) => println!("Error: {}", err),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => break,
            Err(err) => panic!("{}", err),
        }
    }
}
