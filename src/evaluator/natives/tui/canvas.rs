use crate::{
    evaluator::natives::tui::{WIDGETS, Widget, parse_color},
    native_fn, native_fn_with_data,
};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::evaluator::{
    Callable, EvalResult, Evaluator,
    object::{Method, NativeMethod, Object},
    value::Value,
};

use ratatui::style::Color;

// Tui.create_canvas(x, y, width, height) -> Canvas object
native_fn!(
    FnTuiCreateCanvas,
    "tui_create_canvas",
    4,
    |_evaluator, args| {
        let x = if let Value::Num(n) = args[0] {
            n.0 as u16
        } else {
            return Ok(Value::Null);
        };
        let y = if let Value::Num(n) = args[1] {
            n.0 as u16
        } else {
            return Ok(Value::Null);
        };
        let width = if let Value::Num(n) = args[2] {
            n.0 as u16
        } else {
            return Ok(Value::Null);
        };
        let height = if let Value::Num(n) = args[3] {
            n.0 as u16
        } else {
            return Ok(Value::Null);
        };

        let canvas_data = Rc::new(RefCell::new(CanvasData {
            x,
            y,
            width,
            height,
            x_bounds: (0.0, 100.0),
            y_bounds: (0.0, 100.0),
            commands: Vec::new(),
        }));

        let mut methods: HashMap<String, Method> = HashMap::new();

        methods.insert(
            "line".into(),
            Method::Native(NativeMethod::new(
                Rc::new(CanvasLineMethod {
                    data: Rc::clone(&canvas_data),
                }),
                false,
            )),
        );

        methods.insert(
            "circle".into(),
            Method::Native(NativeMethod::new(
                Rc::new(CanvasCircleMethod {
                    data: Rc::clone(&canvas_data),
                }),
                false,
            )),
        );

        methods.insert(
            "rectangle".into(),
            Method::Native(NativeMethod::new(
                Rc::new(CanvasRectangleMethod {
                    data: Rc::clone(&canvas_data),
                }),
                false,
            )),
        );

        methods.insert(
            "points".into(),
            Method::Native(NativeMethod::new(
                Rc::new(CanvasPointsMethod {
                    data: Rc::clone(&canvas_data),
                }),
                false,
            )),
        );

        methods.insert(
            "set_bounds".into(),
            Method::Native(NativeMethod::new(
                Rc::new(CanvasSetBoundsMethod {
                    data: Rc::clone(&canvas_data),
                }),
                false,
            )),
        );

        methods.insert(
            "clear".into(),
            Method::Native(NativeMethod::new(
                Rc::new(CanvasClearMethod {
                    data: Rc::clone(&canvas_data),
                }),
                false,
            )),
        );

        methods.insert(
            "render".into(),
            Method::Native(NativeMethod::new(
                Rc::new(CanvasRenderMethod {
                    data: Rc::clone(&canvas_data),
                }),
                false,
            )),
        );

        Ok(Value::Obj(Rc::new(Object::new("Canvas".into(), methods))))
    }
);

pub struct CanvasData {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
    commands: Vec<CanvasCommand>,
}

#[derive(Clone)]
pub enum CanvasCommand {
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: Color,
    },
    Circle {
        x: f64,
        y: f64,
        radius: f64,
        color: Color,
    },
    Rectangle {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: Color,
    },
    Points {
        points: Vec<(f64, f64)>,
        color: Color,
    },
}

// Canvas method implementations using the macro

native_fn_with_data!(
    CanvasLineMethod,
    "line",
    5,
    CanvasData,
    |_evaluator, args, data| {
        let x1 = if let Value::Num(n) = args[0] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let y1 = if let Value::Num(n) = args[1] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let x2 = if let Value::Num(n) = args[2] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let y2 = if let Value::Num(n) = args[3] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let color = match &args[4] {
            Value::Str(s) => parse_color(&s.borrow()),
            _ => Color::White,
        };

        data.borrow_mut().commands.push(CanvasCommand::Line {
            x1,
            y1,
            x2,
            y2,
            color,
        });

        Ok(Value::Null)
    }
);

native_fn_with_data!(
    CanvasCircleMethod,
    "circle",
    4,
    CanvasData,
    |_evaluator, args, data| {
        let x = if let Value::Num(n) = args[0] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let y = if let Value::Num(n) = args[1] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let radius = if let Value::Num(n) = args[2] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let color = match &args[3] {
            Value::Str(s) => parse_color(&s.borrow()),
            _ => Color::White,
        };

        data.borrow_mut().commands.push(CanvasCommand::Circle {
            x,
            y,
            radius,
            color,
        });

        Ok(Value::Null)
    }
);

native_fn_with_data!(
    CanvasRectangleMethod,
    "rectangle",
    5,
    CanvasData,
    |_evaluator, args, data| {
        let x = if let Value::Num(n) = args[0] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let y = if let Value::Num(n) = args[1] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let width = if let Value::Num(n) = args[2] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let height = if let Value::Num(n) = args[3] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let color = match &args[4] {
            Value::Str(s) => parse_color(&s.borrow()),
            _ => Color::White,
        };

        data.borrow_mut().commands.push(CanvasCommand::Rectangle {
            x,
            y,
            width,
            height,
            color,
        });

        Ok(Value::Null)
    }
);

native_fn_with_data!(
    CanvasPointsMethod,
    "points",
    2,
    CanvasData,
    |_evaluator, args, data| {
        let points = match &args[0] {
            Value::List(list) => {
                let mut pts = Vec::new();
                for item in list.borrow().iter() {
                    if let Value::List(pair) = item {
                        let pair_borrow = pair.borrow();
                        if pair_borrow.len() == 2 {
                            if let (Value::Num(x), Value::Num(y)) =
                                (&pair_borrow[0], &pair_borrow[1])
                            {
                                pts.push((x.0, y.0));
                            }
                        }
                    }
                }
                pts
            }
            _ => return Ok(Value::Null),
        };

        let color = match &args[1] {
            Value::Str(s) => parse_color(&s.borrow()),
            _ => Color::White,
        };

        data.borrow_mut()
            .commands
            .push(CanvasCommand::Points { points, color });

        Ok(Value::Null)
    }
);

native_fn_with_data!(
    CanvasSetBoundsMethod,
    "set_bounds",
    4,
    CanvasData,
    |_evaluator, args, data| {
        let x_min = if let Value::Num(n) = args[0] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let x_max = if let Value::Num(n) = args[1] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let y_min = if let Value::Num(n) = args[2] {
            n.0
        } else {
            return Ok(Value::Null);
        };
        let y_max = if let Value::Num(n) = args[3] {
            n.0
        } else {
            return Ok(Value::Null);
        };

        let mut d = data.borrow_mut();
        d.x_bounds = (x_min, x_max);
        d.y_bounds = (y_min, y_max);

        Ok(Value::Null)
    }
);

native_fn_with_data!(
    CanvasClearMethod,
    "clear",
    0,
    CanvasData,
    |_evaluator, _args, data| {
        data.borrow_mut().commands.clear();
        Ok(Value::Null)
    }
);

native_fn_with_data!(
    CanvasRenderMethod,
    "render",
    0,
    CanvasData,
    |_evaluator, _args, data| {
        let d = data.borrow();

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::Canvas {
                x: d.x,
                y: d.y,
                width: d.width,
                height: d.height,
                x_bounds: d.x_bounds,
                y_bounds: d.y_bounds,
                commands: d.commands.clone(),
            });
        });

        Ok(Value::Null)
    }
);
