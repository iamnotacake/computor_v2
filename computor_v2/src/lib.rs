#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

use std::collections::HashMap;

pub type Context = HashMap<String, Expr>;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    // Neg(Box<Expr>),
    // Add(Box<Expr>, Box<Expr>),
}

pub fn process_line(line: &str, context: &mut Context) {
    match grammar::NumberParser::new().parse(line) {
        Ok(expr) => println!("OK: {:?}", expr),
        Err(err) => println!("Error: {}", err),
    }
}
