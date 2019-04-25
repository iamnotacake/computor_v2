#![feature(box_syntax, box_patterns, slice_patterns)]

extern crate computor_v1;

use std::env::args;
use computor_v1::parser;
use computor_v1::Expr;

#[derive(Debug)]
pub struct Poly {
    pub list: Vec<(u32, f64)>,
}

impl Poly {
    pub fn from_expr(expr: &Expr) -> Result<Poly, ()> {
        use std::collections::HashMap;
        use Expr::*;

        if let Equation(box Add(vec), box Number(_)) = expr {
            let mut list: Vec<(u32, f64)> = vec![];

            for item in vec {
                match item {
                    Number(x) => list.push((0, *x)),
                    Variable(_) => list.push((1, 1.0)),
                    Neg(box Variable(_)) => list.push((1, -1.0)),
                    Neg(box Mul(x, mul)) => match mul.as_slice() {
                        [Variable(_)] => list.push((1, -(*x))),
                        [Pow(box Expr::Variable(_), box Number(n))] => {
                            list.push((*n as u32, -(*x)))
                        }
                        _ => return Err(()),
                    },
                    Mul(x, mul) => match mul.as_slice() {
                        [Variable(_)] => list.push((1, *x)),
                        [Pow(box Expr::Variable(_), box Number(n))] => list.push((*n as u32, *x)),
                        _ => return Err(()),
                    },
                    Pow(box Variable(_), box Number(n)) => list.push((*n as u32, 1.0)),
                    Pow(box Neg(box Variable(_)), box Number(n)) => list.push((*n as u32, -1.0)),
                    Neg(box Pow(box Variable(_), box Number(n))) => list.push((*n as u32, -1.0)),
                    _ => return Err(()),
                }
            }

            let mut map: HashMap<u32, f64> = HashMap::new();
            for (a, b) in list {
                if map.contains_key(&a) {
                    *map.get_mut(&a).unwrap() += b;
                } else {
                    map.insert(a, b);
                }
            }
            let mut list: Vec<(u32, f64)> = map.drain().collect();
            list.sort_by(|(a, _), (b, _)| b.cmp(a));

            Ok(Poly { list })
        } else if let Equation(box Mul(n, vec), box Number(_)) = expr {
            match vec.as_slice() {
                [Variable(_)] => Ok(Poly {
                    list: vec![(1, *n)],
                }),
                [Pow(box Variable(_), box Number(x))] => Ok(Poly {
                    list: vec![(*x as u32, *n)],
                }),

                _ => Err(()),
            }
        } else if let Equation(box Variable(_), box Number(_)) = expr {
            Ok(Poly { list: vec![(1, 0.0)] })
        } else if let Equation(box Neg(box Variable(_)), box Number(_)) = expr {
            Ok(Poly { list: vec![(1, 0.0)] })
        } else {
            Err(())
        }
    }
}

fn solve_quad(a: f64, b: f64, c: f64) {
    let discriminant = b * b - 4. * a * c;
    println!("Discriminant is {}", discriminant);

    if discriminant > 0. {
        let d = discriminant.sqrt();

        let x = (-b - d) / (2. * a);
        println!("x = {:.3} / {:.3} = {:.3}", -b - d, 2. * a, x);

        let x = (-b + d) / (2. * a);
        println!("x = {:.3} / {:.3} = {:.3}", -b + d, 2. * a, x);
    } else if discriminant == 0. {
        let x = (-b) / (2. * a);
        println!("x = {:.3} / {:.3} = {:.3}", -b, 2. * a, x);
    } else {
        println!("No real solutions. Let's use imagination");

        let d = (-discriminant).sqrt();

        let x = (-b - d) / (2. * a);
        println!("x = {:.3}*i / {:.3} = {:.3}*i", -b - d, 2. * a, x);

        let x = (-b + d) / (2. * a);
        println!("x = {:.3}*i / {:.3} = {:.3}*i", -b + d, 2. * a, x);
    }
}

fn main() {
    if let Some(equation) = args().nth(1) {
        println!(">>> {}", equation);
        match parser::equation(&equation) {
            Ok(mut expr) => {
                println!("==> {}", expr.to_string());
                expr = expr.flatten();
                expr = expr.simplify();
                println!("==> {}", expr.to_string());
                expr = expr.move_to_left();
                expr = expr.flatten();
                expr = expr.simplify();
                expr = expr.flatten();
                expr = expr.simplify();
                println!("==> {}", expr.to_string());
                // println!("{:#?}", expr);
                if let Ok(Poly { list }) = Poly::from_expr(&expr) {
                    println!("Polynomial: {:?}", list);
                    match list.as_slice() {
                        [(1, _)] => println!("x = 0"),
                        [(2, a)] => solve_quad(*a, 0., 0.),
                        [(1, b), (0, c)] => println!("x = {}", -c / b),
                        [(2, a), (1, b)] => solve_quad(*a, *b, 0.),
                        [(2, a), (0, c)] => solve_quad(*a, 0., *c),
                        [(2, a), (1, b), (0, c)] => solve_quad(*a, *b, *c),
                        _ => println!("I can't solve that!"),
                    }
                } else {
                    println!("Not a polynomial!");
                }
            }
            Err(err) => {
                println!("    {}^", " ".repeat(err.column - 1));
                println!("{}", err);
            }
        }
    } else {
        eprintln!("Error: No input given. Give me some argument with equation.");
    }
}
