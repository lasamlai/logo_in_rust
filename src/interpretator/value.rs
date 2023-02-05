use core::ops::{Div, Mul, Sub};
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use Value::*;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Value {
    Str(String),
    Num(f32),
    List(Vec<String>),
    Void,
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
        match v {
            Str(s) => match s.parse() {
                Ok(n) => Ok(n),
                Err(_) => Err("Expected number".to_string()),
            },
            Num(n) => Ok(n),
            _ => Err("Type error: It is not a number!".to_string()),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Value {
        Num(if value { 1. } else { 0. })
    }
}

impl TryFrom<Value> for bool {
    type Error = String;
    fn try_from(v: Value) -> Result<bool, String> {
        v.try_into().map(|n: f32| n != 0.)
    }
}

impl From<Vec<String>> for Value {
    fn from(value: Vec<String>) -> Value {
        List(value)
    }
}

impl TryFrom<Value> for Vec<String> {
    type Error = String;
    fn try_from(v: Value) -> Result<Self, String> {
        if let List(l) = v {
            Ok(l)
        } else {
            Err("Type error: It is not a list!".to_string())
        }
    }
}

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Str(s) => formatter.write_fmt(format_args!("{s}")),
            Num(n) => formatter.write_fmt(format_args!("{n}")),
            List(l) => formatter.write_fmt(format_args!("[ {} ]", l.join(" "))),
            Void => formatter.write_fmt(format_args!("Void")),
        }
    }
}
