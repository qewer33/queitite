use std::{
    collections::HashMap, rc::Rc, thread, time::{Duration, SystemTime, UNIX_EPOCH}
};

use ordered_float::OrderedFloat;

use crate::{
    evaluator::{Callable, EvalResult, Evaluator, object::{Method, NativeMethod, Object}, value::Value},
    native_fn,
};

pub fn native_sys() -> Value {
    let mut methods: HashMap<String, Method> = HashMap::new();

    methods.insert(
        "clock".into(),
        Method::Native(NativeMethod::new(Rc::new(FnSysClock), false)),
    );
    methods.insert(
        "sleep".into(),
        Method::Native(NativeMethod::new(Rc::new(FnSysSleep), false)),
    );

    Value::Obj(Rc::new(Object::new("Sys".into(), methods)))
}

native_fn!(FnSysClock, "sys_clock", 0, |_evaluator, _args| {
    let start = SystemTime::now();
    let from_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time should go forward");
    Ok(Value::Num(OrderedFloat(from_epoch.as_millis() as f64)))
});

// sleep(ms)
native_fn!(FnSysSleep, "sys_sleep", 1, |_evaluator, args| {
    if let Value::Num(millis) = args[0] {
        thread::sleep(Duration::from_millis(millis.0 as u64));
    }
    Ok(Value::Null)
});
