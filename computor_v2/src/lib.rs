#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate lalrpop_util;
extern crate failure;
#[macro_use]
extern crate failure_derive;

lalrpop_mod!(pub grammar);

use std::collections::HashMap;
use std::fmt;
use std::string::ToString;

pub type Context = HashMap<String, Expr>;

#[derive(Clone, Debug)]
pub enum Expr {
    Real(f64),
    Var(String),
    Matrix(Vec<Vec<f64>>),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
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
    #[fail(display = "invalid matrix entered, every row must have same length")]
    InvalidMatrix,
}

impl Expr {
    pub fn run(self, context: &mut Context) -> Result<Expr, ExprError> {
        match self {
            Expr::Real(x) => Ok(Expr::Real(x)),
            Expr::Var(name) => match context.get(&name) {
                Some(expr) => Ok(expr.clone()),
                None => Err(ExprError::UndefinedVariable { name: name.clone() }),
            },
            Expr::Matrix(_) => unimplemented!(),
            Expr::Neg(box x) => x.run(context)?.neg(context),
            Expr::Add(box x, box y) => x.run(context)?.add(y.run(context)?, context),
            Expr::Mul(box x, box y) => x.run(context)?.mul(y.run(context)?, context),
            Expr::Div(box x, box y) => x.run(context)?.div(y.run(context)?, context),
            Expr::Rem(box x, box y) => x.run(context)?.rem(y.run(context)?, context),
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

    pub fn rem(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Real(x), Expr::Real(y)) if y == 0.0 => Err(ExprError::DivisionByZero),
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x % y)),
            _ => unimplemented!("rem !Real !Real"),
        }
    }

    pub fn pow(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x.powf(y))),
            _ => unimplemented!("pow !Real !Real"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Real(ref x) => write!(f, "{}", x),
            Expr::Var(ref x) => write!(f, "{}", x),
            Expr::Matrix(ref x) => write!(f, "{:?}", x),
            Expr::Neg(ref x) => write!(f, "-{}", x),
            Expr::Add(ref x, ref y) => write!(f, "({} + {})", x, y),
            Expr::Mul(ref x, ref y) => write!(f, "({} * {})", x, y),
            Expr::Div(ref x, ref y) => write!(f, "({} / {})", x, y),
            Expr::Rem(ref x, ref y) => write!(f, "({} % {})", x, y),
            Expr::Pow(ref x, ref y) => write!(f, "({} ^ {})", x, y),
        }
    }
}

fn validate_matrix(expr: &Expr) -> bool {
    match expr {
        Expr::Real(_) => true,
        Expr::Var(_) => true,
        Expr::Matrix(ref x) => {
            let len = x[0].len();

            x.iter().skip(1).all(|v| v.len() == len)
        }
        Expr::Neg(ref x) => validate_matrix(x),
        Expr::Add(ref x, ref y) => validate_matrix(x) && validate_matrix(y),
        Expr::Mul(ref x, ref y) => validate_matrix(x) && validate_matrix(y),
        Expr::Div(ref x, ref y) => validate_matrix(x) && validate_matrix(y),
        Expr::Rem(ref x, ref y) => validate_matrix(x) && validate_matrix(y),
        Expr::Pow(ref x, ref y) => validate_matrix(x) && validate_matrix(y),
    }
}

pub fn parse(line: &str) -> Result<Expr, ExprError> {
    match grammar::AddSubParser::new().parse(line) {
        Ok(expr) => {
            if validate_matrix(&expr) {
                Ok(expr)
            } else {
                Err(ExprError::InvalidMatrix)
            }
        }
        Err(err) => Err(ExprError::ParseError {
            err: err.to_string(),
        }),
    }
}
