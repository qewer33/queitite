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

// Tui.create_text_input(x, y, width, placeholder) -> TextInput object
native_fn!(
    FnTuiCreateTextInput,
    "tui_create_text_input",
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
        let placeholder = match &args[3] {
            Value::Str(s) => s.borrow().clone(),
            _ => String::new(),
        };

        let input_data = Rc::new(RefCell::new(TextInputData {
            x,
            y,
            width,
            content: String::new(),
            cursor: 0,
            placeholder,
            focused: false,
            style: TextInputStyle::default(),
        }));

        let mut methods: HashMap<String, Method> = HashMap::new();

        methods.insert(
            "get_text".into(),
            Method::Native(NativeMethod::new(
                Rc::new(TextInputGetTextMethod {
                    data: Rc::clone(&input_data),
                }),
                false,
            )),
        );

        methods.insert(
            "set_text".into(),
            Method::Native(NativeMethod::new(
                Rc::new(TextInputSetTextMethod {
                    data: Rc::clone(&input_data),
                }),
                false,
            )),
        );

        methods.insert(
            "handle_key".into(),
            Method::Native(NativeMethod::new(
                Rc::new(TextInputHandleKeyMethod {
                    data: Rc::clone(&input_data),
                }),
                false,
            )),
        );

        methods.insert(
            "clear".into(),
            Method::Native(NativeMethod::new(
                Rc::new(TextInputClearMethod {
                    data: Rc::clone(&input_data),
                }),
                false,
            )),
        );

        methods.insert(
            "set_focused".into(),
            Method::Native(NativeMethod::new(
                Rc::new(TextInputSetFocusedMethod {
                    data: Rc::clone(&input_data),
                }),
                false,
            )),
        );

        methods.insert(
            "set_style".into(),
            Method::Native(NativeMethod::new(
                Rc::new(TextInputSetStyleMethod {
                    data: Rc::clone(&input_data),
                }),
                false,
            )),
        );

        methods.insert(
            "render".into(),
            Method::Native(NativeMethod::new(
                Rc::new(TextInputRenderMethod {
                    data: Rc::clone(&input_data),
                }),
                false,
            )),
        );

        Ok(Value::Obj(Rc::new(Object::new(
            "TextInput".into(),
            methods,
        ))))
    }
);

#[derive(Clone)]
pub struct TextInputData {
    x: u16,
    y: u16,
    width: u16,
    content: String,
    cursor: usize,
    placeholder: String,
    focused: bool,
    style: TextInputStyle,
}

#[derive(Clone)]
pub struct TextInputStyle {
    pub fg: Color,
    pub bg: Color,
    pub border_color: Color,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            fg: Color::White,
            bg: Color::Black,
            border_color: Color::Cyan,
        }
    }
}

// Method implementations using the macro

native_fn_with_data!(
    TextInputGetTextMethod,
    "get_text",
    0,
    TextInputData,
    |_evaluator, _args, data| {
        let d = data.borrow();
        Ok(Value::Str(Rc::new(RefCell::new(d.content.clone()))))
    }
);

native_fn_with_data!(
    TextInputSetTextMethod,
    "set_text",
    1,
    TextInputData,
    |_evaluator, args, data| {
        let text = match &args[0] {
            Value::Str(s) => s.borrow().clone(),
            _ => return Ok(Value::Null),
        };

        let mut d = data.borrow_mut();
        d.content = text;
        d.cursor = d.content.chars().count();

        Ok(Value::Null)
    }
);

native_fn_with_data!(
    TextInputHandleKeyMethod,
    "handle_key",
    1,
    TextInputData,
    |_evaluator, args, data| {
        let key = match &args[0] {
            Value::Str(s) => s.borrow().clone(),
            _ => return Ok(Value::Null),
        };

        let mut d = data.borrow_mut();
        let cursor = d.cursor.clone();

        match key.as_str() {
            "Backspace" => {
                if cursor > 0 {
                    let mut chars: Vec<char> = d.content.chars().collect();
                    chars.remove(cursor - 1);
                    d.content = chars.into_iter().collect();
                    d.cursor -= 1;
                }
            }
            "Space" => {
                d.content.insert(cursor, ' ');
                d.cursor += 1;
            }
            "Delete" => {
                let char_count = d.content.chars().count();
                if cursor < char_count {
                    let mut chars: Vec<char> = d.content.chars().collect();
                    chars.remove(cursor);
                    d.content = chars.into_iter().collect();
                }
            }
            "Left" => {
                if cursor > 0 {
                    d.cursor -= 1;
                }
            }
            "Right" => {
                if cursor < d.content.chars().count() {
                    d.cursor += 1;
                }
            }
            "Home" => {
                d.cursor = 0;
            }
            "End" => {
                d.cursor = d.content.chars().count();
            }
            // Don't process special keys
            "Up" | "Down" | "Enter" | "Esc" | "Tab" | "PageUp" | "PageDown" => {}
            // Everything else is a printable character
            _ => {
                let mut chars: Vec<char> = d.content.chars().collect();
                for c in key.chars() {
                    chars.insert(cursor, c);
                    d.cursor += 1;
                }
                d.content = chars.into_iter().collect();
            }
        }

        Ok(Value::Null)
    }
);

native_fn_with_data!(
    TextInputClearMethod,
    "clear",
    0,
    TextInputData,
    |_evaluator, _args, data| {
        let mut d = data.borrow_mut();
        d.content.clear();
        d.cursor = 0;
        Ok(Value::Null)
    }
);

native_fn_with_data!(
    TextInputSetFocusedMethod,
    "set_focused",
    1,
    TextInputData,
    |_evaluator, args, data| {
        let focused = match &args[0] {
            Value::Bool(b) => *b,
            _ => return Ok(Value::Null),
        };

        data.borrow_mut().focused = focused;
        Ok(Value::Null)
    }
);

native_fn_with_data!(
    TextInputSetStyleMethod,
    "set_style",
    3,
    TextInputData,
    |_evaluator, args, data| {
        let fg = match &args[0] {
            Value::Str(s) => parse_color(&s.borrow()),
            _ => Color::White,
        };

        let bg = match &args[1] {
            Value::Str(s) => parse_color(&s.borrow()),
            Value::Null => Color::Reset,
            _ => Color::Reset,
        };

        let border = match &args[2] {
            Value::Str(s) => parse_color(&s.borrow()),
            _ => Color::Cyan,
        };

        let mut d = data.borrow_mut();
        d.style.fg = fg;
        d.style.bg = bg;
        d.style.border_color = border;

        Ok(Value::Null)
    }
);

native_fn_with_data!(
    TextInputRenderMethod,
    "render",
    0,
    TextInputData,
    |_evaluator, _args, data| {
        let d = data.borrow();

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::TextInput {
                x: d.x,
                y: d.y,
                width: d.width,
                content: d.content.clone(),
                cursor: d.cursor,
                placeholder: d.placeholder.clone(),
                focused: d.focused,
                style: d.style.clone(),
            });
        });

        Ok(Value::Null)
    }
);
