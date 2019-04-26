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
    Complex(f64, f64),
    Var(String),
    Lambda(Vec<String>, Box<Expr>),
    Call(String, Vec<Expr>),
    Matrix(Vec<Vec<f64>>),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    MatrixMul(Box<Expr>, Box<Expr>),
    AssignVar(String, Box<Expr>),
    AssignFunc(Box<Expr>, Box<Expr>),
}

#[derive(Fail, Debug)]
pub enum ExprError {
    #[fail(display = "parse error: {}", err)]
    ParseError { err: String },
    #[fail(display = "undefined variable or function: {}", name)]
    UndefinedVariable { name: String },
    #[fail(display = "division by zero")]
    DivisionByZero,
    #[fail(display = "invalid matrix entered, every row must have same length")]
    InvalidMatrix,
    #[fail(display = "calculation error: {}", err)]
    CalcError { err: String },
    #[fail(display = "bad number of args for function '{}'", func)]
    BadArgsCount { func: String },
    #[fail(display = "recursion is too deep :c")]
    RecursiveRecursion,
}

impl Expr {
    pub fn run(self, context: &mut Context, level: usize) -> Result<Expr, ExprError> {
        let level = level + 1;

        if level > 100 {
            return Err(ExprError::RecursiveRecursion);
        }

        match self {
            Expr::Real(_) => Ok(self),
            Expr::Complex(_, _) => Ok(self),
            Expr::Var(name) => match context.get(&name) {
                Some(expr) => Ok(expr.clone()),
                None => Err(ExprError::UndefinedVariable { name: name.clone() }),
            },
            Expr::Lambda(args, expr) => unimplemented!(),

            Expr::Call(name, args) => {
                if let Some(Expr::Lambda(names, expr)) = context.get(&name) {
                    if names.len() != args.len() {
                        return Err(ExprError::BadArgsCount { func: name.clone() });
                    }

                    let mut context = context.clone();

                    for (arg_name, arg_val) in names.iter().zip(args) {
                        context.insert(arg_name.to_string(), arg_val);
                    }

                    Ok(expr.clone().run(&mut context, level)?)
                } else {
                    Err(ExprError::UndefinedVariable { name: name.clone() })
                }
            }

            Expr::Matrix(_) => Ok(self),
            Expr::Neg(box x) => x.run(context, level)?.neg(context),
            Expr::Add(box x, box y) => x.run(context, level)?.add(y.run(context, level)?, context),
            Expr::Mul(box x, box y) => x.run(context, level)?.mul(y.run(context, level)?, context),
            Expr::Div(box x, box y) => x.run(context, level)?.div(y.run(context, level)?, context),
            Expr::Rem(box x, box y) => x.run(context, level)?.rem(y.run(context, level)?, context),
            Expr::Pow(box x, box y) => x.run(context, level)?.pow(y.run(context, level)?, context),
            Expr::MatrixMul(box x, box y) => {
                x.run(context, level)?.mmul(y.run(context, level)?, context)
            }
            Expr::AssignVar(name, box expr) => {
                let expr = expr.run(context, level)?;
                context.insert(name, expr.clone());

                Ok(expr)
            }
            Expr::AssignFunc(box name_args, box expr) => {
                if let Expr::Call(name, args) = name_args {
                    let mut new_args = vec![];

                    for arg in args {
                        if let Expr::Var(n) = arg {
                            new_args.push(n);
                        } else {
                            return Err(ExprError::ParseError {
                                err: "func definition could contain only variables".into(),
                            });
                        }
                    }

                    // dbg!((&name, &new_args, &expr));
                    context.insert(name, Expr::Lambda(new_args, box expr.clone()));

                    Ok(expr)
                } else {
                    Err(ExprError::ParseError {
                        err: "some strange things happened".into(),
                    })
                }
            }
        }
    }

    pub fn neg(self, context: &mut Context) -> Result<Expr, ExprError> {
        match self {
            Expr::Real(x) => Ok(Expr::Real(-x)),
            Expr::Complex(x, y) => Ok(Expr::Complex(-x, -y)),
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
            (Expr::Complex(a, b), Expr::Complex(x, y)) => Ok(Expr::Complex(a + x, b + y)),
            (Expr::Real(a), Expr::Complex(x, y)) | (Expr::Complex(x, y), Expr::Real(a)) => {
                Ok(Expr::Complex(a + x, y))
            }
            (Expr::Real(y), Expr::Matrix(rows)) | (Expr::Matrix(rows), Expr::Real(y)) => {
                Ok(Expr::Matrix(
                    rows.iter()
                        .map(|row| row.iter().map(|x| x + y).collect())
                        .collect(),
                ))
            }
            _ => Err(ExprError::CalcError {
                err: "add !Real !Real".into(),
            }),
        }
    }

    pub fn mul(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x * y)),
            (Expr::Complex(a, b), Expr::Complex(x, y)) => {
                Ok(Expr::Complex(a * x - b * y, a * y + b * x))
            }
            (Expr::Real(a), Expr::Complex(x, y)) | (Expr::Complex(x, y), Expr::Real(a)) => {
                Ok(Expr::Complex(a * x, a * y))
            }
            (Expr::Real(y), Expr::Matrix(rows)) | (Expr::Matrix(rows), Expr::Real(y)) => {
                Ok(Expr::Matrix(
                    rows.iter()
                        .map(|row| row.iter().map(|x| x * y).collect())
                        .collect(),
                ))
            }
            _ => Err(ExprError::CalcError {
                err: "mul !Real !Real".into(),
            }),
        }
    }

    pub fn div(self, other: Expr, context: &mut Context) -> Result<Expr, ExprError> {
        match (self, other) {
            (_, Expr::Real(y)) if y == 0.0 => Err(ExprError::DivisionByZero),
            (Expr::Real(x), Expr::Real(y)) => Ok(Expr::Real(x / y)),
            (Expr::Complex(a, b), Expr::Complex(x, y)) => {
                let a = (a, b);
                let b = (x, y);
                let conj = (b.0, -b.1);

                let top = (a.0 * conj.0 - a.1 * conj.1, a.0 * conj.1 + a.1 * conj.0);
                let bot = (b.0 * conj.0 - b.1 * conj.1, b.0 * conj.1 + b.1 * conj.0);

                assert_eq!(bot.1, 0.0);

                Ok(Expr::Complex(top.0 / bot.0, top.1 / bot.0))
            }
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
            (Expr::Matrix(rows), Expr::Real(y)) => Ok(Expr::Matrix(
                rows.iter()
                    .map(|row| row.iter().map(|x| x.powf(y)).collect())
                    .collect(),
            )),
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
            Expr::Complex(ref x, ref y) => {
                if *y < 0.0 {
                    write!(f, "{} - {}i", x, -y)
                } else {
                    write!(f, "{} + {}i", x, y)
                }
            }
            Expr::Var(ref x) => write!(f, "{}", x),
            Expr::Lambda(ref args, ref expr) => write!(f, "({}) => {}", args.join(", "), expr),
            Expr::Call(ref func, ref args) => write!(f, "({})({})", func, "..."),
            Expr::Matrix(ref x) => write!(f, "{:?}", x),
            Expr::Neg(ref x) => write!(f, "-{}", x),
            Expr::Add(ref x, ref y) => write!(f, "({} + {})", x, y),
            Expr::Mul(ref x, ref y) => write!(f, "({} * {})", x, y),
            Expr::Div(ref x, ref y) => write!(f, "({} / {})", x, y),
            Expr::Rem(ref x, ref y) => write!(f, "({} % {})", x, y),
            Expr::Pow(ref x, ref y) => write!(f, "({} ^ {})", x, y),
            Expr::MatrixMul(ref x, ref y) => write!(f, "({} ** {})", x, y),
            Expr::AssignVar(ref name, ref val) => write!(f, "{} = {}", name, val),
            Expr::AssignFunc(_, _) => {
                // write!(f, "{}({}) = {}", name, args.join(", "), body)
                write!(f, "(function)",)
            }
        }
    }
}

fn validate_matrix(expr: &Expr) -> bool {
    match expr {
        Expr::Real(_) => true,
        Expr::Complex(_, _) => true,
        Expr::Var(_) => true,
        Expr::Lambda(_, ref expr) => validate_matrix(expr),
        Expr::Call(_, ref args) => args.iter().all(|expr| validate_matrix(expr)),
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
        Expr::AssignFunc(_, ref body) => validate_matrix(body),
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
