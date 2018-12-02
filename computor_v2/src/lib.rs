#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

use std::collections::HashMap;

pub type Context = HashMap<String, Expr>;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Var(String),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

pub fn process_line(line: &str, context: &mut Context) {
    match grammar::AddSubParser::new().parse(line) {
        Ok(expr) => println!("OK: {:?}", expr),
        Err(err) => println!("Error: {}", err),
    }
}
