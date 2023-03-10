use crate::interpretator::value::Value;
use crate::parser::Exp::*;
use crate::parser::OP::*;
use crate::unsee::Unsee;
use std::collections::HashMap;

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

fn get_value(txt: &str) -> Option<Exp> {
    if txt == "repcount" || txt == "#" {
        return Some(Var("repcount".to_string()));
    }
    if let Some(name) = txt.strip_prefix(':') {
        Some(Var(name.to_string()))
    } else if let Some(name) = txt.strip_prefix('"') {
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
    Empty,
    Val(Exp),
    Op(Exp, OP),
}

struct ExpParser {
    state: ExpState,
}

impl ExpParser {
    fn new() -> ExpParser {
        ExpParser {
            state: ExpState::Empty,
        }
    }

    fn can_eat_val(&self) -> bool {
        use ExpState::*;
        match self.state {
            Empty | Op(_, _) => true,
            Val(_) => false,
        }
    }

    fn shift_op(&mut self, op: OP) -> bool {
        use ExpState::*;
        match self.state.clone() {
            Val(v) => {
                self.state = Op(v, op);
                true
            }
            _ => false,
        }
    }

    fn shift_val(&mut self, rhs: Exp) -> bool {
        use ExpState::*;
        match self.state.clone() {
            Empty => {
                self.state = Val(rhs);
                true
            }
            Op(v, op) => {
                self.state = Val(Oper(op, Box::new(v), Box::new(rhs)));
                true
            }
            _ => false,
        }
    }

    fn get_value(self) -> Exp {
        use ExpState::*;
        match self.state {
            Val(v) => v,
            Empty => panic!("I do not have value!"),
            Op(_, _) => panic!("Expected second argument"),
        }
    }
}

fn parse_expr(procs: &HashMap<String, usize>, iter: &mut Unsee<&str>) -> Exp {
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
    body: Vec<String>,
}

impl Procedure {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_body(&self) -> Vec<&str> {
        self.body.iter().map(AsRef::as_ref).collect()
    }

    pub fn get_argv(&self) -> Vec<&str> {
        self.vars.iter().map(AsRef::as_ref).collect()
    }

    pub fn signature(&self) -> usize {
        self.vars.len()
    }
}

fn procedure_args(iter: &mut Unsee<&str>) -> Vec<String> {
    let mut vars = vec![];
    loop {
        let txt = iter.next().unwrap();
        if let Some(name) = txt.strip_prefix(':') {
            vars.push(name.to_string());
        } else {
            iter.unsee(txt);
            break;
        }
    }
    vars
}

fn procedure_body(iter: &mut Unsee<&str>) -> Vec<String> {
    let mut body = vec![];
    loop {
        match iter.next() {
            Some("end") | Some("END") => break,
            None => panic!("Expceted 'end'"),
            Some(txt) => body.push(txt.to_string()),
        }
    }
    body
}

fn parse_procedure(iter: &mut Unsee<&str>) -> Procedure {
    let name = iter.next().unwrap().to_string();
    let vars = procedure_args(iter);
    let body = procedure_body(iter);
    Procedure { name, vars, body }
}

pub enum Stat {
    ProcDef(Procedure),
    Exp(Exp),
}

pub fn parse_statement(procs: &HashMap<String, usize>, iter: &mut Unsee<&str>) -> Option<Stat> {
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
