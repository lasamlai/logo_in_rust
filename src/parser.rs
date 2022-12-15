use crate::interpretator::value::Value;
use crate::parser::Exp::*;
use crate::parser::OP::*;
use std::collections::HashMap;
use std::str::Split;

#[derive(Clone)]
pub enum OP {
    Sub,
    Mul,
    Div,
    Le,
}

#[derive(Clone)]
pub enum Exp {
    Call(String, Vec<Exp>),
    Oper(OP, Box<Exp>, Box<Exp>),
    Const(Value),
    Var(String),
}

pub struct StackIter<'a> {
    stack: Vec<&'a str>,
    iter: Split<'a, &'a str>,
}

impl<'a> StackIter<'a> {
    pub fn wrap(iter: Split<'a, &'a str>) -> StackIter<'a> {
        StackIter::<'a> {
            stack: vec![],
            iter,
        }
    }

    pub fn unsee(&mut self, s: &'a str) {
        self.stack.push(s);
    }
}

impl<'a> Iterator for StackIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        match self.stack.pop() {
            None => self.iter.next(),
            Some(s) => Some(s),
        }
    }
}

fn get_value(txt: &str) -> Option<Exp> {
    if txt == "repcount" || txt == "#" {
        return Some(Var("repcount".to_string()));
    }
    if txt.chars().next().unwrap() == ':' {
        let name = &txt[1..].to_string();
        Some(Var(name.to_string()))
    } else if txt.chars().next().unwrap() == '"' {
        let name = &txt[1..].to_string();
        Some(Const(Value::Str(name.to_string())))
    } else {
        match txt.parse() {
            Ok(n) => Some(Const(Value::Num(n))),
            Err(_) => None,
        }
    }
}

#[derive(Clone)]
enum ExpState {
    ExpNone,
    ExpVal(Exp),
    ExpOp(Exp, OP),
}

struct ExpParser {
    state: ExpState,
}

impl ExpParser {
    fn new() -> ExpParser {
        ExpParser {
            state: ExpState::ExpNone,
        }
    }

    fn can_eat_val(&self) -> bool {
        use ExpState::*;
        match self.state {
            ExpNone | ExpOp(_, _) => true,
            ExpVal(_) => false,
        }
    }

    fn shift_op(&mut self, op: OP) -> bool {
        use ExpState::*;
        match self.state.clone() {
            ExpVal(v) => {
                self.state = ExpOp(v, op);
                true
            }
            _ => false,
        }
    }

    fn shift_val(&mut self, rhs: Exp) -> bool {
        use ExpState::*;
        match self.state.clone() {
            ExpNone => {
                self.state = ExpVal(rhs);
                true
            }
            ExpOp(v, op) => {
                self.state = ExpVal(Oper(op, Box::new(v), Box::new(rhs)));
                true
            }
            _ => false,
        }
    }

    fn get_value(self) -> Exp {
        use ExpState::*;
        match self.state {
            ExpVal(v) => v,
            ExpNone => panic!("I do not have value!"),
            ExpOp(_, _) => panic!("Expected second argument"),
        }
    }
}

fn parse_expr(procs: &HashMap<String, usize>, iter: &mut StackIter) -> Exp {
    let mut parser = ExpParser::new();
    loop {
        match iter.next() {
            Some("(") => {
                if !parser.can_eat_val() {
                    iter.unsee("(");
                    break parser.get_value();
                }
                let x = parse_expr(procs, iter);
                if iter.next() != Some(")") {
                    panic!("Where is close bracket `)`?");
                }
                parser.shift_val(x);
            }
            Some("[") => {
                // TODO: nested lists
                if !parser.can_eat_val() {
                    iter.unsee("[");
                    break parser.get_value();
                }
                let mut list = vec![];
                let x = loop {
                    match iter.next() {
                        Some("]") => break Const(Value::List(list)),
                        Some(x) => list.push(x.to_string()),
                        None => panic!(),
                    }
                };
                parser.shift_val(x);
            }
            Some("*") => {
                parser.shift_op(Mul);
            }
            Some("-") => {
                parser.shift_op(Sub);
            }
            Some("/") => {
                parser.shift_op(Div);
            }
            Some("<") => {
                parser.shift_op(Le);
            }
            Some(txt) => {
                if !parser.can_eat_val() {
                    iter.unsee(txt);
                    break parser.get_value();
                }
                match get_value(txt) {
                    Some(x) => {
                        let ok = parser.shift_val(x);

                        if !ok {
                            iter.unsee(txt);
                            return parser.get_value();
                        }
                    }
                    None => match procs.get(txt) {
                        Some(n) => {
                            let args = (0..*n).map(|_| parse_expr(procs, iter)).collect();
                            parser.shift_val(Exp::Call(txt.to_string(), args));
                        }
                        None => {
                            iter.unsee(txt);
                            return parser.get_value();
                        }
                    },
                }
            }
            None => return parser.get_value(),
        }
    }
}

#[derive(Clone)]
pub struct Procedure {
    name: String,
    vars: Vec<String>,
    body: String,
}

impl Procedure {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn get_argv(&self) -> Vec<String> {
        self.vars.clone()
    }

    pub fn signature(&self) -> usize {
        self.vars.len()
    }
}

fn procedure_args(iter: &mut StackIter) -> Vec<String> {
    let mut vars = vec![];
    loop {
        let txt = iter.next().unwrap();
        if txt.chars().next().unwrap() == ':' {
            let name = &txt[1..].to_string();
            vars.push(name.to_string());
        } else {
            iter.unsee(txt);
            break;
        }
    }
    vars
}

fn procedure_body(iter: &mut StackIter) -> String {
    let mut body = vec![];
    loop {
        match iter.next() {
            Some("end") | Some("END") => break,
            None => panic!("Expceted 'end'"),
            Some(txt) => body.push(txt.to_string()),
        }
    }
    body.join(" ")
}

fn parse_procedure(iter: &mut StackIter) -> Procedure {
    let name = iter.next().unwrap().to_string();
    let vars = procedure_args(iter);
    let body = procedure_body(iter);
    Procedure { name, vars, body }
}

pub enum Stat {
    ProcDef(Procedure),
    Exp(Exp),
}

pub fn parse_statement(procs: &HashMap<String, usize>, iter: &mut StackIter) -> Option<Stat> {
    match iter.next() {
        None => None,
        Some("to") | Some("TO") | Some("To") => {
            let proc = parse_procedure(iter);
            Some(Stat::ProcDef(proc))
        }
        Some(txt) => {
            iter.unsee(txt);
            Some(Stat::Exp(parse_expr(procs, iter)))
        }
    }
}
