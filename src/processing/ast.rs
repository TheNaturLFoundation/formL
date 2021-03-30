use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    Number(i32),
    BinaryOp(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Copy, Clone)]
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}

impl Expr {
    pub fn evaluate(&self) -> i32 {
        use self::Expr::*;
        match *self {
            Number(i) => i,
            BinaryOp(ref a, op, ref b) => {
                match op {
                    Operator::Mul => {
                        a.evaluate() * b.evaluate()
                    }
                    Operator::Div => {
                        a.evaluate() / b.evaluate()
                    }
                    Operator::Add => {
                        a.evaluate() + b.evaluate()
                    }
                    Operator::Sub => {
                        a.evaluate() - b.evaluate()
                    }
                }
            }
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(x) => write!(fmt, "{:?}", x),
            BinaryOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Operator::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
