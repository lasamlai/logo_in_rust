use core::ops::{Div, Mul, Sub};
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use Value::*;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Value {
    Str(String),
    Num(f32),
    Bool(bool),
    List(Vec<String>),
    Void,
}

impl Value {
    pub fn to_bool(self) -> bool {
        if let Bool(s) = self {
            s
        } else {
            panic!("Expect Boolean!")
        }
    }

    pub fn to_list(self) -> Vec<String> {
        if let List(s) = self {
            s
        } else {
            panic!("Expect list!")
        }
    }
}

impl Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Num(a), Num(b)) => Num(a - b),
            _ => panic!(),
        }
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Num(a), Num(b)) => Num(a * b),
            _ => panic!(),
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Num(a), Num(b)) => Num(a / b),
            _ => panic!(),
        }
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Value {
        Num(value)
    }
}

impl TryFrom<Value> for f32 {
    type Error = String;
    fn try_from(v: Value) -> Result<f32, String> {
        if let Num(n) = v {
            Ok(n)
        } else {
            Err("Type error: It is not a number!".to_string())
        }
    }
}

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Str(s) => formatter.write_fmt(format_args!("{}", s)),
            Num(n) => formatter.write_fmt(format_args!("{}", n)),
            Bool(b) => formatter.write_fmt(format_args!("{}", b)),
            List(l) => formatter.write_fmt(format_args!("[ {} ]", l.join(" "))),
            Void => formatter.write_fmt(format_args!("Void")),
        }
    }
}
