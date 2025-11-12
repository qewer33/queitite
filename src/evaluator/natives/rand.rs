use std::{collections::HashMap, rc::Rc};

use ordered_float::OrderedFloat;
use rand::Rng;

use crate::{
    evaluator::{
        Callable, EvalResult, Evaluator,
        object::{Method, NativeMethod, Object},
        value::Value,
    },
    native_fn,
};

pub fn native_rand() -> Value {
    let mut methods: HashMap<String, Method> = HashMap::new();

    methods.insert(
        "num".into(),
        Method::Native(NativeMethod::new(Rc::new(FnRandNum), false)),
    );

    Value::Obj(Rc::new(Object::new("Rand".into(), methods)))
}

// rand() -> Num (0..1)
native_fn!(FnRandNum, "rand_num", 0, |_evaluator, _args| {
    let mut rng = rand::rng();
    Ok(Value::Num(OrderedFloat(rng.random())))
});
