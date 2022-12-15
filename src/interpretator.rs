use crate::interpretator::Value::*;
use crate::parser::parse_statement;
use crate::parser::Exp;
use crate::parser::Procedure;
use crate::parser::StackIter;
use crate::parser::Stat;
use crate::parser::OP;
use crate::robot::Robot;
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
}

pub enum ExpReturn {
    Exit(Value),
    Result(Value),
}

impl ExpReturn {
    fn and_then<F: FnOnce(Value) -> ExpReturn>(self, f: F) -> ExpReturn {
        match self {
            ExpReturn::Result(x) => f(x),
            exit => exit,
        }
    }

    fn exp_return(self) -> ExpReturn {
        match self {
            ExpReturn::Exit(v) => ExpReturn::Result(v),
            res => res,
        }
    }
}

fn interete_exp(ctx: &mut Context, exp: Exp) -> ExpReturn {
    use crate::parser::Exp::*;
    match exp {
        Oper(m, e1, e2) => interete_exp(ctx, *e1).and_then(|x| {
            interete_exp(ctx, *e2).and_then(|y| match m {
                OP::Sub => ExpReturn::Result(x - y),
                OP::Mul => ExpReturn::Result(x * y),
                OP::Div => ExpReturn::Result(x / y),
                OP::Le => ExpReturn::Result(Bool(x < y)),
            })
        }),
        Call(pr, args) => interpretr_call(ctx, pr, args),
        Const(v) => ExpReturn::Result(v.clone()),
        Var(s) => ExpReturn::Result(ctx.vars.get(&s).cloned().unwrap()),
    }
}

fn interprete_run(ctx: &mut Context, code: Value) -> ExpReturn {
    let code = code.to_list().join(" ");
    interete(ctx, &mut StackIter::wrap(code.split(" ")))
}

fn interpretr_proc(ctx: &mut Context, proc: Procedure, vals: VecDeque<Value>) -> ExpReturn {
    let argv: Vec<String> = proc.get_argv();
    let vals: Vec<(String, Value)> = zip(argv.clone(), vals).collect();

    // save vars
    let save: Vec<(String, Option<Value>)> = argv
        .into_iter()
        .map(|k| {
            let v = ctx.vars.get(&k).cloned();
            (k, v)
        })
        .collect();

    vals.into_iter().for_each(|(name, val)| {
        ctx.vars.insert(name.to_string(), val);
    });
    let res = interete(ctx, &mut StackIter::wrap(proc.get_body().split(" ")));

    //restore vars
    save.into_iter().for_each(|(k, o)| {
        match o {
            Some(o) => ctx.vars.insert(k, o),
            None => ctx.vars.remove(&k),
        };
    });

    return res.exp_return();
}

fn interpretr_call(ctx: &mut Context, pr: String, args: Vec<Exp>) -> ExpReturn {
    let mut vals: VecDeque<Value> = VecDeque::new();
    for e in args.into_iter() {
        match interete_exp(ctx, e) {
            ExpReturn::Exit(v) => return ExpReturn::Exit(v),
            ExpReturn::Result(r) => vals.push_back(r),
        }
    }
    match &pr[..] {
        "stop" => return ExpReturn::Exit(Value::Void),
        "output" => return ExpReturn::Exit(vals.pop_front().unwrap()),
        _ => (),
    }
    match &pr[..] {
        "fd" | "forward" => ctx.robot.forward(vals.pop_front().unwrap().to_num()),
        "bk" | "back" => ctx.robot.back(vals.pop_front().unwrap().to_num()),
        "rt" | "right" => ctx
            .robot
            .right(vals.pop_front().unwrap().to_num() * PI / 180.0),
        "lt" | "left" => ctx
            .robot
            .left(vals.pop_front().unwrap().to_num() * PI / 180.0),
        "setcolor" | "setpencolor" => ctx.robot.setpencolor(vals.pop_front().unwrap().to_string()),
        "home" => ctx.robot.home(),
        "label" => ctx.robot.label(vals.pop_front().unwrap().to_string()),
        "setlabelheight" => ctx.robot.setlabelheight(vals.pop_front().unwrap().to_num()),
        "penup" | "pu" => ctx.robot.penup(),
        "pendown" | "pd" => ctx.robot.pendown(),
        "wait" => println!("wait {:?}", vals.pop_front().unwrap()),
        "clean" => ctx.robot.clean(),
        "clearscreen" | "cs" => ctx.robot.clearscreen(),
        "window" => println!("window"),
        "hideturtle" | "ht" => println!("Hide the turtle!"),
        "showturtle" | "st" => println!("Show the turtle!"),
        "pick" => {
            let vs: Vec<String> = vals.pop_front().unwrap().to_list();
            return ExpReturn::Result(Value::Str(
                vs.choose(&mut rand::thread_rng()).unwrap().clone(),
            ));
        }
        "random" => {
            let n: i32 = vals.pop_front().unwrap().to_num() as i32;
            return ExpReturn::Result(Value::Num(rand::thread_rng().gen_range(0..n) as f32));
        }
        "sentence" => {
            let mut l1 = vals.pop_front().unwrap().to_list();
            let mut l2 = vals.pop_front().unwrap().to_list();
            l1.append(&mut l2);
            return ExpReturn::Result(Value::List(l1));
        }
        "pr" | "print" => {
            println!("{}", vals[0]);
        }
        "run" => {
            return interprete_run(ctx, vals.pop_front().unwrap());
        }

        "repeat" => {
            let num = vals.pop_front().unwrap().to_num() as i32;
            let code = vals.pop_front().unwrap();
            for i in 0..num {
                ctx.vars.insert("repcount".to_string(), Num(i as f32));
                let v = interprete_run(ctx, code.clone());
                if let ExpReturn::Exit(res) = v {
                    return ExpReturn::Exit(res);
                }
            }
        }
        "if" => {
            let que = vals.pop_front().unwrap().to_bool();
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
                .expect(&format!("Unknowc command: {}", s));
            return interpretr_proc(ctx, proc, vals);
        }
    };
    ExpReturn::Result(Value::Void)
}

fn interete_proc(ctx: &mut Context, exp: Exp) -> ExpReturn {
    use crate::parser::Exp::*;
    match exp {
        Call(pr, args) => interpretr_call(ctx, pr, args),
        exp => {
            interete_exp(ctx, exp);
            ExpReturn::Result(Value::Void)
        }
    }
}

fn interete(ctx: &mut Context, iter: &mut StackIter) -> ExpReturn {
    loop {
        match parse_statement(&ctx.signs, iter) {
            None => return ExpReturn::Result(Value::Void),
            Some(Stat::ProcDef(proc)) => {
                ctx.signs.insert(proc.get_name(), proc.signature());
                ctx.procs.insert(proc.get_name(), proc);
            }
            Some(Stat::Exp(e)) => match interete_proc(ctx, e) {
                ExpReturn::Result(Value::Void) => continue,
                ExpReturn::Result(v) => panic!("Don't know what to do with {}", v),
                ExpReturn::Exit(v) => return ExpReturn::Exit(v),
            },
        }
    }
}

pub fn inter(data: String) -> svg::Document {
    let mut ctx = Context::new();
    interete(&mut ctx, &mut StackIter::wrap(data.split(" ")));
    ctx.plot()
}
