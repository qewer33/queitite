mod macros;
mod math;
mod p5;
mod rand;
mod sys;
mod term;
mod tui;

use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
    str::FromStr,
};

use crate::{
    evaluator::{
        Evaluator,
        env::{Env, EnvPtr},
        runtime_err::{ErrKind, EvalResult, RuntimeErr, RuntimeEvent},
        value::{Callable, Value},
    },
    native_fn,
};

pub struct Natives;

impl Natives {
    pub fn get_natives() -> EnvPtr {
        let natives = Env::new();

        // global functions
        natives
            .borrow_mut()
            .define("print".into(), Value::Callable(Rc::new(FnPrint)));
        natives
            .borrow_mut()
            .define("println".into(), Value::Callable(Rc::new(FnPrintln)));
        natives
            .borrow_mut()
            .define("read".into(), Value::Callable(Rc::new(FnRead)));
        natives
            .borrow_mut()
            .define("err".into(), Value::Callable(Rc::new(FnErr)));

        // global objects
        natives.borrow_mut().define("Sys".into(), sys::native_sys());
        natives
            .borrow_mut()
            .define("Rand".into(), rand::native_rand());
        natives
            .borrow_mut()
            .define("Math".into(), math::native_math());
        natives
            .borrow_mut()
            .define("Term".into(), term::native_term());
        natives.borrow_mut().define("Tui".into(), tui::native_tui());
        natives.borrow_mut().define("P5".into(), p5::native_p5());

        natives
    }
}

// print(expr)
native_fn!(FnPrint, "print", 1, |_evaluator, args, _cursor| {
    print!("{}", args[0]);
    Ok(Value::Null)
});

// println(expr)
native_fn!(FnPrintln, "println", 1, |_evaluator, args, _cursor| {
    println!("{}", args[0]);
    Ok(Value::Null)
});

// read() -> Str
native_fn!(FnRead, "read", 0, |_evaluator, _args, cursor| {
    io::stdout().flush().map_err(|err| {
        RuntimeEvent::error(
            ErrKind::IO,
            format!("failed to flush stdout: {}", err),
            cursor,
        )
    })?;
    let mut string = String::new();
    io::stdin().read_line(&mut string).map_err(|err| {
        RuntimeEvent::error(ErrKind::IO, format!("failed to read line: {}", err), cursor)
    })?;
    Ok(Value::Str(Rc::new(RefCell::new(string.trim().to_string()))))
});

// err(kind, msg) -> throws a runtime error of given kind
native_fn!(FnErr, "err", 2, |_evaluator, args, cursor| {
    let kind_str = args[0].check_str(cursor, Some("kind".into()))?;
    let kind = ErrKind::from_str(kind_str.borrow().as_str())
        .map_err(|_| RuntimeEvent::error(ErrKind::Value, "invalid error kind".into(), cursor))?;
    let msg = args[1].check_str(cursor, Some("message".into()))?;
    Err(RuntimeEvent::Err(RuntimeErr::new(
        kind,
        msg.borrow().clone(),
        cursor,
    )))
});
