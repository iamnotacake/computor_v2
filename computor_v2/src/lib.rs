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
    Lambda(Vec<String>, Box<Expr>),
    Matrix(Vec<Vec<f64>>),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    MatrixMul(Box<Expr>, Box<Expr>),
    AssignVar(String, Box<Expr>),
    AssignFunc(String, Vec<String>, Box<Expr>),
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
    #[fail(display = "calculation error: {}", err)]
    CalcError { err: String },
}

impl Expr {
    pub fn run(self, context: &mut Context) -> Result<Expr, ExprError> {
        match self {
            Expr::Real(x) => Ok(Expr::Real(x)),
            Expr::Var(name) => match context.get(&name) {
                Some(expr) => Ok(expr.clone()),
                None => Err(ExprError::UndefinedVariable { name: name.clone() }),
            },
            Expr::Lambda(args, expr) => unimplemented!(),
            Expr::Matrix(_) => Ok(self),
            Expr::Neg(box x) => x.run(context)?.neg(context),
            Expr::Add(box x, box y) => x.run(context)?.add(y.run(context)?, context),
            Expr::Mul(box x, box y) => x.run(context)?.mul(y.run(context)?, context),
            Expr::Div(box x, box y) => x.run(context)?.div(y.run(context)?, context),
            Expr::Rem(box x, box y) => x.run(context)?.rem(y.run(context)?, context),
            Expr::Pow(box x, box y) => x.run(context)?.pow(y.run(context)?, context),
            Expr::MatrixMul(box x, box y) => x.run(context)?.mmul(y.run(context)?, context),
            Expr::AssignVar(name, box expr) => {
                let expr = expr.run(context)?;
                context.insert(name, expr.clone());

                Ok(expr)
            }
            Expr::AssignFunc(name, args, box expr) => {
                context.insert(name, Expr::Lambda(args, box expr.clone()));

                Ok(expr)
            }
        }
    }

    pub fn neg(self, context: &mut Context) -> Result<Expr, ExprError> {
        match self {
            Expr::Real(x) => Ok(Expr::Real(-x)),
            Expr::Matrix(rows) => Ok(Expr::Matrix(
                rows.iter()
                    .map(|row| row.iter().map(|x| -x).collect())
                    .collect(),
            )),
            _ => Err(ExprError::CalcError {
                err: "neg !Real".into(),
            }),
        }
    }

    pub fn add(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x + y)),
            (Expr::Real(y), Expr::Matrix(rows)) => Ok(Expr::Matrix(
                rows.iter()
                    .map(|row| row.iter().map(|x| x + y).collect())
                    .collect(),
            )),
            (Expr::Matrix(rows), Expr::Real(y)) => Ok(Expr::Matrix(
                rows.iter()
                    .map(|row| row.iter().map(|x| x + y).collect())
                    .collect(),
            )),
            _ => Err(ExprError::CalcError {
                err: "add !Real !Real".into(),
            }),
        }
    }

    pub fn mul(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x * y)),
            (Expr::Real(y), Expr::Matrix(rows)) => Ok(Expr::Matrix(
                rows.iter()
                    .map(|row| row.iter().map(|x| x * y).collect())
                    .collect(),
            )),
            (Expr::Matrix(rows), Expr::Real(y)) => Ok(Expr::Matrix(
                rows.iter()
                    .map(|row| row.iter().map(|x| x * y).collect())
                    .collect(),
            )),
            _ => Err(ExprError::CalcError {
                err: "mul !Real !Real".into(),
            }),
        }
    }

    pub fn div(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (_, Expr::Real(y)) if y == 0.0 => Err(ExprError::DivisionByZero),
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x / y)),
            (Expr::Matrix(rows), Expr::Real(y)) => Ok(Expr::Matrix(
                rows.iter()
                    .map(|row| row.iter().map(|x| x / y).collect())
                    .collect(),
            )),
            _ => Err(ExprError::CalcError {
                err: "div !Real !Real".into(),
            }),
        }
    }

    pub fn rem(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (_, Expr::Real(y)) if y == 0.0 => Err(ExprError::DivisionByZero),
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x % y)),
            (Expr::Matrix(rows), Expr::Real(y)) => Ok(Expr::Matrix(
                rows.iter()
                    .map(|row| row.iter().map(|x| x % y).collect())
                    .collect(),
            )),
            _ => Err(ExprError::CalcError {
                err: "mod !Real !Real".into(),
            }),
        }
    }

    pub fn pow(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x.powf(y))),
            _ => Err(ExprError::CalcError {
                err: "pow !Real !Real".into(),
            }),
        }
    }

    pub fn mmul(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Matrix(m1), Expr::Matrix(m2)) => {
                if m1[0].len() != m2.len() {
                    return Err(ExprError::CalcError {
                        err: "matrices have invalid size".into(),
                    });
                }

                let mut res = vec![vec![0.0; m2[0].len()]; m1.len()];

                // m1 => n * m
                // m2 => m * p

                let n = m1.len();
                let m = m1[0].len();
                let p = m2[0].len();

                for i in 0..n {
                    for j in 0..p {
                        res[i][j] = (0..m).map(|k| m1[i][k] * m2[k][j]).sum();
                    }
                }

                Ok(Expr::Matrix(res))
            }
            _ => Err(ExprError::CalcError {
                err: "matrix multiplication works only on matrices".into(),
            }),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Real(ref x) => write!(f, "{}", x),
            Expr::Var(ref x) => write!(f, "{}", x),
            Expr::Lambda(ref args, ref expr) => write!(f, "({}) => {}", args.join(", "), expr),
            Expr::Matrix(ref x) => write!(f, "{:?}", x),
            Expr::Neg(ref x) => write!(f, "-{}", x),
            Expr::Add(ref x, ref y) => write!(f, "({} + {})", x, y),
            Expr::Mul(ref x, ref y) => write!(f, "({} * {})", x, y),
            Expr::Div(ref x, ref y) => write!(f, "({} / {})", x, y),
            Expr::Rem(ref x, ref y) => write!(f, "({} % {})", x, y),
            Expr::Pow(ref x, ref y) => write!(f, "({} ^ {})", x, y),
            Expr::MatrixMul(ref x, ref y) => write!(f, "({} ** {})", x, y),
            Expr::AssignVar(ref name, ref val) => write!(f, "{} = {}", name, val),
            Expr::AssignFunc(ref name, ref args, ref body) => {
                write!(f, "{}({}) = {}", name, args.join(", "), body)
            }
        }
    }
}

fn validate_matrix(expr: &Expr) -> bool {
    match expr {
        Expr::Real(_) => true,
        Expr::Var(_) => true,
        Expr::Lambda(_, ref expr) => validate_matrix(expr),
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
        Expr::MatrixMul(ref x, ref y) => validate_matrix(x) && validate_matrix(y),
        Expr::AssignVar(_, ref val) => validate_matrix(val),
        Expr::AssignFunc(_, _, ref body) => validate_matrix(body),
    }
}

pub fn parse(line: &str) -> Result<Expr, ExprError> {
    match grammar::RootExprParser::new().parse(line) {
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
