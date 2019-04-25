#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(char),
    Neg(Box<Expr>),
    Add(Vec<Expr>),
    Mul(f64, Vec<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Equation(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn flatten(self) -> Expr {
        use Expr::*;

        match self {
            Number(_) => self,

            Variable(_) => self,

            Neg(_) => self,

            Add(_) => self,

            Mul(mut x, mut vec) => {
                let mut new = vec![];

                if let Number(a) = vec[0] {
                    x *= a;
                    vec.remove(0);
                }

                for mut item in vec {
                    item = item.flatten();

                    if let Mul(n, inner) = item {
                        x *= n;
                        new.extend(inner);
                    } else {
                        new.push(item);
                    }
                }

                if new.len() > 0 {
                    Mul(x, new)
                } else {
                    Number(x)
                }
            }

            Pow(x, y) => Pow(box x.flatten(), box y.flatten()),

            Equation(x, y) => Equation(box x.flatten(), box y.flatten()),
        }
    }

    pub fn simplify(self) -> Expr {
        use Expr::*;

        match self {
            Number(_) => self,

            Variable(_) => self,

            Neg(box Number(x)) => Number(-x),
            Neg(box Variable(_)) => self,
            Neg(box Neg(x)) => x.simplify(),
            Neg(box Add(mut vec)) => Add(vec.drain(..).map(|x| Neg(box x)).collect()).simplify(),
            Neg(box Equation(..)) => unreachable!(),
            Neg(x) => Neg(box x.simplify()),

            Add(vec) => {
                let mut new = vec![];

                for mut item in vec {
                    item = item.simplify();

                    if let Add(inner) = item {
                        new.extend(inner);
                    } else {
                        new.push(item.simplify());
                    }
                }

                let (vec, mut new) = (new, vec![]);

                let mut sum = vec![];
                let mut other = vec![];

                for mut item in vec {
                    item = item.simplify();

                    match item {
                        Number(x) => sum.push(x),
                        _ => other.push(item),
                    }
                }

                new.extend(other);

                if sum.len() > 0 {
                    let sum = sum.iter().sum();

                    if sum != 0.0 {
                        new.push(Number(sum));
                    }
                }

                if new.len() > 1 {
                    Add(new)
                } else if new.len() == 1 {
                    new.swap_remove(0)
                } else {
                    Number(0.0)
                }
            }

            Mul(x, _) if x == 0.0 => Number(0.0),
            Mul(mut x, vec) => {
                let mut new = vec![];

                let mut vars = vec![];
                let mut other = vec![];

                for mut item in vec {
                    item = item.simplify();

                    match item {
                        Number(a) => x *= a,
                        Variable(x) => vars.push(x),
                        _ => other.push(item),
                    }
                }

                if vars.len() > 0 {
                    new.extend(vars.iter().map(|&x| Variable(x)));
                }

                new.extend(other);

                if new.len() == 0 {
                    Number(x)
                } else if new.len() > 0 && x != 1.0 {
                    Mul(x, new)
                } else {
                    new.swap_remove(0)
                }
            }

            Pow(box Number(x), box Number(y)) => Number(x.powf(y)),
            Pow(x, box Number(y)) => if y == 1.0 { x.simplify() } else { Pow(box x.simplify(), box Number(y)) },
            Pow(x, y) => Pow(box x.simplify(), box y.simplify()),

            Equation(l, r) => Equation(box l.simplify(), box r.simplify()),
        }
    }

    pub fn move_to_left(self) -> Expr {
        use Expr::*;

        if let Equation(l, r) = self {
            Equation(box Add(vec![*l, Neg(r)]), box Number(0.0))
        } else {
            panic!("Attempt to move_to_left() on non-equation");
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        use Expr::*;

        match self {
            Number(x) => x.to_string(),

            Variable(x) => x.to_string(),

            Neg(x @ box Number(_)) |
            Neg(x @ box Variable(_)) |
            Neg(x @ box Mul(_, _)) |
            Neg(x @ box Pow(_, _)) => "-".to_string() + &x.to_string(),
            Neg(x) => "-".to_string() + "(" + &x.to_string() + ")",

            Add(vec) => {
                let mut res = vec[0].to_string();

                for i in &vec[1..] {
                    let s = i.to_string();

                    if s.starts_with("-") {
                        res += " - ";
                        res += &s[1..];
                    } else {
                        res += " + ";
                        res += &s;
                    }
                }
                res
            }

            Mul(x, vec) => {
                let mut res = "".to_string();

                if *x != 1.0 {
                    res += &x.to_string();
                    res += "*";
                }

                let first = &vec[0];

                match first {
                    Add(_) => {
                        res += "(";
                        res += &first.to_string();
                        res += ")";
                    }
                    _ => res += &first.to_string(),
                }

                for i in &vec[1..] {
                    res += "*";

                    match i {
                        Add(_) => {
                            res += "(";
                            res += &i.to_string();
                            res += ")";
                        }
                        _ => res += &i.to_string(),
                    }
                }
                res
            }

            Pow(x @ box Add(_), y @ box Add(_)) |
            Pow(x @ box Add(_), y @ box Mul(..)) |
            Pow(x @ box Mul(..), y @ box Add(_)) |
            Pow(x @ box Mul(..), y @ box Mul(..)) => {
                "(".to_string() + &x.to_string() + ")^(" + &y.to_string() + ")"
            }
            Pow(x @ box Add(_), y) | Pow(x @ box Mul(..), y) => {
                "(".to_string() + &x.to_string() + ")^" + &y.to_string()
            }
            Pow(x, y @ box Add(_)) | Pow(x, y @ box Mul(..)) => {
                x.to_string() + "^(" + &y.to_string() + ")"
            }
            Pow(x, y) => x.to_string() + "^" + &y.to_string(),

            Equation(x, y) => x.to_string() + " = " + &y.to_string(),
        }
    }
}
