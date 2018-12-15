#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

use std::collections::HashMap;

pub type Context = HashMap<String, Expr>;

#[derive(Clone, Debug)]
pub enum Expr {
    Number(f64),
    Var(String),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn run(self, context: &mut Context) -> Expr {
        match self {
            Expr::Number(x) => Expr::Number(x),
            Expr::Var(name) => context.get(&name).unwrap().clone(),
            Expr::Neg(box x) => x.run(context).neg(context),
            Expr::Add(box x, box y) => x.run(context).add(y.run(context), context),
            Expr::Mul(box x, box y) => x.run(context).mul(y.run(context), context),
            Expr::Div(box x, box y) => x.run(context).div(y.run(context), context),
            Expr::Pow(box x, box y) => x.run(context).pow(y.run(context), context),
        }
    }

    pub fn neg(self, context: &mut Context) -> Expr {
        match self {
            Expr::Number(x) => Expr::Number(-x),
            _ => unimplemented!("neg !Number"),
        }
    }

    pub fn add(self, other: Expr, context: &mut Context) -> Expr {
        match (self, other) {
            (Expr::Number(x), Expr::Number(y)) => Expr::Number(x + y),
            _ => unimplemented!("add !Number !Number"),
        }
    }

    pub fn mul(self, other: Expr, context: &mut Context) -> Expr {
        match (self, other) {
            (Expr::Number(x), Expr::Number(y)) => Expr::Number(x * y),
            _ => unimplemented!("mul !Number !Number"),
        }
    }

    pub fn div(self, other: Expr, context: &mut Context) -> Expr {
        match (self, other) {
            (Expr::Number(x), Expr::Number(y)) => Expr::Number(x / y),
            _ => unimplemented!("div !Number !Number"),
        }
    }

    pub fn pow(self, other: Expr, context: &mut Context) -> Expr {
        match (self, other) {
            (Expr::Number(x), Expr::Number(y)) => Expr::Number(x.powf(y)),
            _ => unimplemented!("pow !Number !Number"),
        }
    }
}

pub fn parse(line: &str) -> Result<Expr, String> {
    match grammar::AddSubParser::new().parse(line) {
        Ok(expr) => Ok(expr),
        Err(err) => Err(err.to_string()),
    }
}
