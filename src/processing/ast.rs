use std::fmt::{Debug, Formatter, Error};

pub enum Operator {
    Plus,
    Mul,
    Div,
    Minus,
}

pub enum SetType {
    Real,
    Integer,
    Natural,
    Custom(String)
}

impl Debug for SetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            SetType::Real => write!(f, "ℝ"),
            SetType::Integer => write!(f, "ℤ"),
            SetType::Natural => write!(f, "ℕ"),
            SetType::Custom(s) => write!(f, "{}", s),
        }
    }
}

pub enum Expr {
    Natural(u64),
    Integer(i64),
    Real(f64),
    Identifier(String),
    Assignment(String, Box<Expr>),
    FunctionDefinition(String, SetType, SetType, Box<Expr>),
    BinaryOperator(Box<Expr>, Operator, Box<Expr>),
    Tuple(Vec<Box<Expr>>),
    List(Vec<Box<Expr>>),
}

impl Debug for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Operator::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Plus => write!(fmt, "+"),
            Minus => write!(fmt, "-"),
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Assignment(ref s, ref exp) => write!(fmt, "Assign {:?} to {:?}", *exp, s),
            Natural(x) => write!(fmt, "Natural({:?})", x),
            Integer(x) => write!(fmt, "Integer({:?})", x),
            Real(x) => write!(fmt, "Real({:?})", x),
            BinaryOperator(ref l, ref op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, &op, r),
            Identifier(ref s) => write!(fmt, "{:?}", s),
            FunctionDefinition(ref name, ref ker, ref im, ref exp) => {
                write!(fmt, "Defining {:?} : {:?} -> {:?}\n x |-> {:?}", name, ker, im, exp)
            }
            Tuple(ref t) => {
                write!(fmt, "Tuple(")?;
                for (index, i) in t.iter().enumerate() {
                    write!(fmt, "{:?}{}", i, if index == (t.len() - 1) {""} else {", "})?;
                }
                write!(fmt, ")")
            }
            List(ref t) => {
                write!(fmt, "List[")?;
                for (index, i) in t.iter().enumerate() {
                    write!(fmt, "{:?}{}", i, if index == (t.len() - 1) {""} else {", "})?;
                }
                write!(fmt, "]")
            }
        }
    }
}