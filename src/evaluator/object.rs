use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    evaluator::{
        function::Function,
        runtime_err::{EvalResult, RuntimeEvent},
        value::{Callable, Value},
    },
    lexer::cursor::Cursor,
};

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub methods: HashMap<String, Function>,
}

impl Object {
    pub fn new(name: String, methods: HashMap<String, Function>) -> Self {
        Self { name, methods }
    }

    fn find_method(&self, name: String) -> Option<Function> {
        self.methods.get(&name).cloned()
    }
}

impl Callable for Object {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn arity(&self) -> usize {
        if let Some(init) = self.find_method("init".to_string()) {
            return init.arity();
        }

        0
    }

    fn call(
        &self,
        evaluator: &mut super::Evaluator,
        args: Vec<super::value::Value>,
    ) -> EvalResult<Value> {
        let inst = Value::ObjInstance(Rc::new(RefCell::new(Instance::new(self.clone()))));

        if let Some(init) = self.find_method("init".to_string()) {
            init.bind_method(inst.clone()).call(evaluator, args)?;
        }

        Ok(inst)
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    obj: Object,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(obj: Object) -> Self {
        Self {
            obj,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: String, cursor: Cursor) -> EvalResult<Value> {
        if let Some(val) = self.fields.get(&name) {
            return Ok(val.clone());
        }

        if let Some(func) = self.obj.find_method(name.clone()) {
            let new_func =
                func.bind_method(Value::ObjInstance(Rc::new(RefCell::new(self.clone()))));
            return Ok(Value::Callable(Rc::new(new_func)));
        }

        Err(RuntimeEvent::error(
            format!("undefined property '{}'", name),
            cursor,
        ))
    }

    pub fn set(&mut self, name: String, val: Value) {
        self.fields.insert(name, val);
    }
}

impl ToString for Instance {
    fn to_string(&self) -> String {
        format!("{} instance", self.obj.name)
    }
}
