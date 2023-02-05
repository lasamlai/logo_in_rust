use crate::interpretator::Value::*;
use crate::parser::parse_statement;
use crate::parser::Exp;
use crate::parser::Procedure;
use crate::parser::Stat;
use crate::parser::OP;
use crate::robot::Robot;
use crate::unsee::Unsee;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::f32::consts::PI;
use std::iter::zip;
pub mod value;
use value::*;

pub struct Context {
    robot: Robot,
    vars: HashMap<String, Value>,
    procs: HashMap<String, Procedure>,
    signs: HashMap<String, usize>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            robot: Robot::new(),
            vars: HashMap::new(),
            procs: HashMap::new(),
            signs: Self::init_signatures(),
        }
    }

    pub fn plot(&mut self) -> svg::Document {
        self.robot.plot()
    }

    fn init_signatures() -> HashMap<String, usize> {
        let mut signs: HashMap<String, usize> = HashMap::new();
        signs.insert("stop".to_string(), 0);
        signs.insert("output".to_string(), 1);
        signs.insert("fd".to_string(), 1);
        signs.insert("forward".to_string(), 1);
        signs.insert("bk".to_string(), 1);
        signs.insert("back".to_string(), 1);
        signs.insert("rt".to_string(), 1);
        signs.insert("right".to_string(), 1);
        signs.insert("lt".to_string(), 1);
        signs.insert("left".to_string(), 1);
        signs.insert("setcolor".to_string(), 1);
        signs.insert("setpencolor".to_string(), 1);
        signs.insert("home".to_string(), 0);
        signs.insert("label".to_string(), 1);
        signs.insert("setlabelheight".to_string(), 1);
        signs.insert("penup".to_string(), 0);
        signs.insert("pu".to_string(), 0);
        signs.insert("pendown".to_string(), 0);
        signs.insert("pd".to_string(), 0);
        signs.insert("wait".to_string(), 1);
        signs.insert("clean".to_string(), 0);
        signs.insert("clearscreen".to_string(), 0);
        signs.insert("cs".to_string(), 0);
        signs.insert("window".to_string(), 0);
        signs.insert("hideturtle".to_string(), 0);
        signs.insert("ht".to_string(), 0);
        signs.insert("showturtle".to_string(), 0);
        signs.insert("st".to_string(), 0);
        signs.insert("pr".to_string(), 1);
        signs.insert("print".to_string(), 1);
        signs.insert("repeat".to_string(), 2);
        signs.insert("if".to_string(), 2);
        signs.insert("make".to_string(), 2);

        signs.insert("pick".to_string(), 1);
        signs.insert("random".to_string(), 1);
        signs.insert("sentence".to_string(), 2);

        signs
    }

    fn keep_values_out_of_context(&mut self, names: Vec<String>) -> Vec<(String, Option<Value>)> {
        names
            .into_iter()
            .map(|k| {
                let v = self.vars.remove(&k);
                (k, v)
            })
            .collect()
    }

    fn restore_values_from_context(&mut self, backup: Vec<(String, Option<Value>)>) {
        backup.into_iter().for_each(|(k, o)| {
            match o {
                Some(o) => self.vars.insert(k, o),
                None => self.vars.remove(&k),
            };
        });
    }
}

enum ExpResult {
    Exit(Value),
    Outcome(Value),
}

impl ExpResult {
    fn and_then<F: FnOnce(Value) -> ExpResult>(self, f: F) -> ExpResult {
        match self {
            ExpResult::Outcome(x) => f(x),
            exit => exit,
        }
    }

    fn exp_return(self) -> ExpResult {
        match self {
            ExpResult::Exit(v) => ExpResult::Outcome(v),
            res => res,
        }
    }
}

fn interete_exp(ctx: &mut Context, exp: Exp) -> ExpResult {
    use crate::parser::Exp::*;
    match exp {
        Oper(m, e1, e2) => interete_exp(ctx, *e1).and_then(|x| {
            interete_exp(ctx, *e2).and_then(|y| match m {
                OP::Sub => ExpResult::Outcome(x - y),
                OP::Mul => ExpResult::Outcome(x * y),
                OP::Div => ExpResult::Outcome(x / y),
                OP::Le => ExpResult::Outcome(Value::from(x < y)),
            })
        }),
        Call(pr, args) => interpretr_call(ctx, pr, args),
        Const(v) => ExpResult::Outcome(v),
        Var(s) => ExpResult::Outcome(ctx.vars.get(&s).cloned().unwrap()),
    }
}

fn interprete_run(ctx: &mut Context, code: Value) -> ExpResult {
    let code: Vec<String> = code.try_into().expect("Expect List!");
    let mut unsee = Unsee::wrap(code.iter().map(AsRef::as_ref));
    interete(ctx, &mut unsee)
}

fn interpretr_proc(ctx: &mut Context, proc: Procedure, vals: VecDeque<Value>) -> ExpResult {
    let argv: Vec<String> = proc.get_argv().into_iter().map(str::to_owned).collect();
    let save = ctx.keep_values_out_of_context(argv.clone());

    zip(argv, vals).for_each(|(name, val)| {
        ctx.vars.insert(name, val);
    });
    let res = interete(ctx, &mut Unsee::wrap(proc.get_body().into_iter()));

    ctx.restore_values_from_context(save);
    res.exp_return()
}

fn interpretr_call(ctx: &mut Context, pr: String, args: Vec<Exp>) -> ExpResult {
    let mut vals: VecDeque<Value> = VecDeque::new();
    for e in args.into_iter() {
        match interete_exp(ctx, e) {
            ExpResult::Exit(v) => return ExpResult::Exit(v),
            ExpResult::Outcome(r) => vals.push_back(r),
        }
    }
    match &pr[..] {
        "stop" => return ExpResult::Exit(Value::Void),
        "output" => return ExpResult::Exit(vals.pop_front().unwrap()),
        _ => (),
    }
    match &pr[..] {
        "fd" | "forward" => ctx.robot.forward(
            vals.pop_front()
                .unwrap()
                .try_into()
                .expect("Expected number!"),
        ),
        "bk" | "back" => ctx.robot.back(
            vals.pop_front()
                .unwrap()
                .try_into()
                .expect("Expected number!"),
        ),
        "rt" | "right" => {
            let d: f32 = vals
                .pop_front()
                .unwrap()
                .try_into()
                .expect("Expected number!");
            ctx.robot.right(d * PI / 180.0)
        }

        "lt" | "left" => {
            let d: f32 = vals
                .pop_front()
                .unwrap()
                .try_into()
                .expect("Expected number!");
            ctx.robot.left(d * PI / 180.0)
        }
        "setcolor" | "setpencolor" => ctx.robot.setpencolor(vals.pop_front().unwrap().to_string()),
        "home" => ctx.robot.home(),
        "label" => ctx.robot.label(vals.pop_front().unwrap().to_string()),
        "setlabelheight" => ctx.robot.setlabelheight(
            vals.pop_front()
                .unwrap()
                .try_into()
                .expect("Expected number!"),
        ),
        "penup" | "pu" => ctx.robot.penup(),
        "pendown" | "pd" => ctx.robot.pendown(),
        "wait" => println!("wait {:?}", vals.pop_front().unwrap()),
        "clean" => ctx.robot.clean(),
        "clearscreen" | "cs" => ctx.robot.clearscreen(),
        "window" => println!("window"),
        "hideturtle" | "ht" => println!("Hide the turtle!"),
        "showturtle" | "st" => println!("Show the turtle!"),
        "pick" => {
            let vs: Vec<String> = vals.pop_front().unwrap().try_into().expect("Expect List!");
            return ExpResult::Outcome(Value::Str(
                vs.choose(&mut rand::thread_rng()).unwrap().clone(),
            ));
        }
        "random" => {
            let n: f32 = vals
                .pop_front()
                .unwrap()
                .try_into()
                .expect("Expected number!");
            let n: i32 = n as i32;
            return ExpResult::Outcome(Value::Num(rand::thread_rng().gen_range(0..n) as f32));
        }
        "sentence" => {
            let mut l1: Vec<String> = vals.pop_front().unwrap().try_into().expect("Expect List!");
            let mut l2: Vec<String> = vals.pop_front().unwrap().try_into().expect("Expect List!");
            l1.append(&mut l2);
            return ExpResult::Outcome(Value::List(l1));
        }
        "pr" | "print" => {
            println!("{}", vals[0]);
        }
        "run" => {
            return interprete_run(ctx, vals.pop_front().unwrap());
        }

        "repeat" => {
            let num: f32 = vals
                .pop_front()
                .unwrap()
                .try_into()
                .expect("Expected number!");
            let num: i32 = num as i32;
            let code = vals.pop_front().unwrap();
            for i in 0..num {
                ctx.vars.insert("repcount".to_string(), Num(i as f32));
                let v = interprete_run(ctx, code.clone());
                if let ExpResult::Exit(res) = v {
                    return ExpResult::Exit(res);
                }
            }
        }
        "if" => {
            let que = vals
                .pop_front()
                .unwrap()
                .try_into()
                .expect("Expected boolean!");
            let code = vals.pop_front().unwrap();
            if que {
                return interprete_run(ctx, code);
            }
        }
        "make" => {
            let name: String = vals.pop_front().unwrap().to_string();
            ctx.vars.insert(name, vals.pop_front().unwrap());
        }
        s => {
            let proc = ctx
                .procs
                .get(s)
                .cloned()
                .unwrap_or_else(|| panic!("Unknown command: {}", s));
            return interpretr_proc(ctx, proc, vals);
        }
    };
    ExpResult::Outcome(Value::Void)
}

fn interete(ctx: &mut Context, iter: &mut Unsee<&str>) -> ExpResult {
    loop {
        match parse_statement(&ctx.signs, iter) {
            None => return ExpResult::Outcome(Value::Void),
            Some(Stat::ProcDef(proc)) => {
                ctx.signs
                    .insert(proc.get_name().to_owned(), proc.signature());
                ctx.procs.insert(proc.get_name().to_owned(), proc);
            }
            Some(Stat::Exp(e)) => match interete_exp(ctx, e) {
                ExpResult::Outcome(Value::Void) => continue,
                ExpResult::Outcome(v) => panic!("Don't know what to do with {}", v),
                ExpResult::Exit(v) => return ExpResult::Exit(v),
            },
        }
    }
}

pub fn inter(data: Vec<&str>) -> svg::Document {
    let mut ctx = Context::new();
    interete(&mut ctx, &mut Unsee::wrap(data.iter().map(AsRef::as_ref)));
    ctx.plot()
}
