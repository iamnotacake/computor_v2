#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate lalrpop_util;
extern crate failure;
#[macro_use]
extern crate failure_derive;

lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::string::ToString;

pub type Context = HashMap<String, Expr>;

#[derive(Clone, Debug)]
pub enum Expr {
    Real(f64),
    Var(String),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

#[derive(Fail, Debug)]
pub enum ExprError {
    #[fail(display = "parse error: {}", err)]
    ParseError { err: String },
    #[fail(display = "undefined variable: {}", name)]
    UndefinedVariable { name: String },
    #[fail(display = "division by zero")]
    DivisionByZero,
}

impl Expr {
    pub fn run(self, context: &mut Context) -> Result<Expr, ExprError> {
        match self {
            Expr::Real(x) => Ok(Expr::Real(x)),
            Expr::Var(name) => match context.get(&name) {
                Some(expr) => Ok(expr.clone()),
                None => Err(ExprError::UndefinedVariable { name: name.clone() }),
            },
            Expr::Neg(box x) => x.run(context)?.neg(context),
            Expr::Add(box x, box y) => x.run(context)?.add(y.run(context)?, context),
            Expr::Mul(box x, box y) => x.run(context)?.mul(y.run(context)?, context),
            Expr::Div(box x, box y) => x.run(context)?.div(y.run(context)?, context),
            Expr::Pow(box x, box y) => x.run(context)?.pow(y.run(context)?, context),
        }
    }

    pub fn neg(self, context: &mut Context) -> Result<Expr, ExprError> {
        match self {
            Expr::Real(x) => Ok(Expr::Real(-x)),
            _ => unimplemented!("neg !Real"),
        }
    }

    pub fn add(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x + y)),
            _ => unimplemented!("add !Real !Real"),
        }
    }

    pub fn mul(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x * y)),
            _ => unimplemented!("mul !Real !Real"),
        }
    }

    pub fn div(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Real(x), Expr::Real(y)) if y == 0.0 => Err(ExprError::DivisionByZero),
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x / y)),
            _ => unimplemented!("div !Real !Real"),
        }
    }

    pub fn pow(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x.powf(y))),
            _ => unimplemented!("pow !Real !Real"),
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Real(ref x) => x.to_string(),
            Expr::Var(ref x) => x.clone(),
            Expr::Neg(ref x) => "-".to_string() + &x.to_string(),
            Expr::Add(ref x, ref y) => {
                "(".to_string() + &x.to_string() + " + " + &y.to_string() + ")"
            }
            Expr::Mul(ref x, ref y) => {
                "(".to_string() + &x.to_string() + " * " + &y.to_string() + ")"
            }
            Expr::Div(ref x, ref y) => {
                "(".to_string() + &x.to_string() + " / " + &y.to_string() + ")"
            }
            Expr::Pow(ref x, ref y) => {
                "(".to_string() + &x.to_string() + " ^ " + &y.to_string() + ")"
            }
        }
    }
}

pub fn parse(line: &str) -> Result<Expr, ExprError> {
    match grammar::AddSubParser::new().parse(line) {
        Ok(expr) => Ok(expr),
        Err(err) => Err(ExprError::ParseError {
            err: err.to_string(),
        }),
    }
}
