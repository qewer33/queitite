use std::{collections::HashMap, rc::Rc};

use ordered_float::OrderedFloat;

use crate::{
    evaluator::{
        Callable, EvalResult, Evaluator,
        object::{Method, NativeMethod, Object},
        value::Value,
    },
    native_fn,
};

pub fn native_math() -> Value {
    let mut methods: HashMap<String, Method> = HashMap::new();

    methods.insert(
        "sin".into(),
        Method::Native(NativeMethod::new(Rc::new(FnMathSin), false)),
    );
    methods.insert(
        "cos".into(),
        Method::Native(NativeMethod::new(Rc::new(FnMathCos), false)),
    );

    Value::Obj(Rc::new(Object::new("Rand".into(), methods)))
}

// sin(x) -> Num
native_fn!(FnMathSin, "sin", 1, |_evaluator, args| {
    let x = if let Value::Num(n) = &args[0] {
        n
    } else {
        return Ok(Value::Null);
    };

    Ok(Value::Num(OrderedFloat(x.sin())))
});

// cos(x) -> Num
native_fn!(FnMathCos, "cos", 1, |_evaluator, args| {
    let x = if let Value::Num(n) = &args[0] {
        n
    } else {
        return Ok(Value::Null);
    };

    Ok(Value::Num(OrderedFloat(x.cos())))
});