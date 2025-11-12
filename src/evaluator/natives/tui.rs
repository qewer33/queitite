mod canvas;
mod text_input;

use std::{cell::RefCell, collections::HashMap, io, rc::Rc};

use crate::{
    evaluator::{
        Callable, EvalResult, Evaluator,
        natives::tui::{
            canvas::{CanvasCommand, FnTuiCreateCanvas},
            text_input::{FnTuiCreateTextInput, TextInputStyle},
        },
        object::{Method, NativeMethod, Object},
        value::Value,
    },
    native_fn,
};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
};

pub fn native_tui() -> Value {
    let mut methods: HashMap<String, Method> = HashMap::new();

    methods.insert(
        "init".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiInit), false)),
    );
    methods.insert(
        "cleanup".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiCleanup), false)),
    );
    methods.insert(
        "draw_block".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawBlock), false)),
    );
    methods.insert(
        "draw_text".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawText), false)),
    );
    methods.insert(
        "draw_list".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawList), false)),
    );
    methods.insert(
        "draw_progress".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawProgress), false)),
    );
    methods.insert(
        "clear".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiClear), false)),
    );
    methods.insert(
        "render".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiRender), false)),
    );

    methods.insert(
        "create_canvas".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiCreateCanvas), false)),
    );
    methods.insert(
        "create_text_input".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiCreateTextInput), false)),
    );

    Value::Obj(Rc::new(Object::new("Tui".into(), methods)))
}

// Widget types to accumulate before rendering
#[derive(Clone)]
enum Widget {
    Block {
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        title: String,
        color: Color,
    },
    Text {
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        text: String,
        fg: Color,
        bg: Color,
    },
    List {
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        items: Vec<String>,
        selected: usize,
        color: Color,
        title: String,
    },
    Progress {
        x: u16,
        y: u16,
        width: u16,
        percent: u16,
        label: String,
        color: Color,
    },
    Canvas {
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        x_bounds: (f64, f64),
        y_bounds: (f64, f64),
        commands: Vec<CanvasCommand>,
    },
    TextInput {
        x: u16,
        y: u16,
        width: u16,
        content: String,
        cursor: usize,
        placeholder: String,
        focused: bool,
        style: TextInputStyle,
    },
}

// Global terminal instance and widget buffer
thread_local! {
    static TERMINAL: RefCell<Option<Terminal<CrosstermBackend<io::Stdout>>>> = RefCell::new(None);
    static WIDGETS: RefCell<Vec<Widget>> = RefCell::new(Vec::new());
}

// Tui.init(): initializes the TUI (enters alternate screen, raw mode)
native_fn!(FnTuiInit, "tui_init", 0, |_evaluator, _args| {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    TERMINAL.with(|t| {
        *t.borrow_mut() = Some(terminal);
    });

    Ok(Value::Null)
});

// Tui.cleanup(): cleans up the TUI (exits alternate screen, restores terminal)
native_fn!(FnTuiCleanup, "tui_cleanup", 0, |_evaluator, _args| {
    TERMINAL.with(|t| {
        if let Some(mut terminal) = t.borrow_mut().take() {
            let _ = disable_raw_mode();
            let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
            let _ = terminal.show_cursor();
        }
    });

    Ok(Value::Null)
});

// Tui.clear(): clears the widget buffer (call this at the start of each frame)
native_fn!(FnTuiClear, "tui_clear", 0, |_evaluator, _args| {
    WIDGETS.with(|w| {
        w.borrow_mut().clear();
    });

    Ok(Value::Null)
});

// Tui.render(): renders all accumulated widgets to the screen
native_fn!(FnTuiRender, "tui_render", 0, |_evaluator, _args| {
    let result = TERMINAL.with(|t| -> io::Result<()> {
        if let Some(terminal) = t.borrow_mut().as_mut() {
            terminal.draw(|frame| {
                WIDGETS.with(|w| {
                    for widget in w.borrow().iter() {
                        match widget {
                            Widget::Block {
                                x,
                                y,
                                width,
                                height,
                                title,
                                color,
                            } => {
                                let area = Rect::new(*x, *y, *width, *height);
                                let block = Block::default()
                                    .title(title.clone())
                                    .borders(Borders::ALL)
                                    .border_style(Style::default().fg(*color));
                                frame.render_widget(block, area);
                            }
                            Widget::Text {
                                x,
                                y,
                                width,
                                height,
                                text,
                                fg,
                                bg,
                            } => {
                                let area = Rect::new(*x, *y, *width, *height);
                                let paragraph = Paragraph::new(text.clone())
                                    .style(Style::default().fg(*fg).bg(*bg))
                                    .wrap(Wrap { trim: false });
                                frame.render_widget(paragraph, area);
                            }
                            Widget::List {
                                x,
                                y,
                                width,
                                height,
                                items,
                                selected,
                                color,
                                title,
                            } => {
                                let area = Rect::new(*x, *y, *width, *height);

                                let list_items: Vec<ListItem> = items
                                    .iter()
                                    .enumerate()
                                    .map(|(i, item)| {
                                        let prefix = if i == *selected { "> " } else { "  " };
                                        let style = if i == *selected {
                                            Style::default().fg(*color).add_modifier(Modifier::BOLD)
                                        } else {
                                            Style::default()
                                        };
                                        ListItem::new(format!("{}{}", prefix, item)).style(style)
                                    })
                                    .collect();

                                let list = List::new(list_items).block(
                                    Block::default().title(title.clone()).borders(Borders::ALL),
                                );

                                frame.render_widget(list, area);
                            }
                            Widget::Progress {
                                x,
                                y,
                                width,
                                percent,
                                label,
                                color,
                            } => {
                                let area = Rect::new(*x, *y, *width, 3);
                                let gauge = Gauge::default()
                                    .block(Block::default().borders(Borders::ALL))
                                    .gauge_style(Style::default().fg(*color))
                                    .percent(*percent)
                                    .label(label.clone());
                                frame.render_widget(gauge, area);
                            }
                            Widget::Canvas {
                                x,
                                y,
                                width,
                                height,
                                x_bounds,
                                y_bounds,
                                commands,
                            } => {
                                use ratatui::widgets::canvas::{
                                    Canvas as RatatuiCanvas, Circle, Line, Points, Rectangle,
                                };

                                let area = Rect::new(*x, *y, *width, *height);

                                let canvas = RatatuiCanvas::default()
                                    .x_bounds([x_bounds.0, x_bounds.1])
                                    .y_bounds([y_bounds.0, y_bounds.1])
                                    .paint(|ctx| {
                                        for cmd in commands {
                                            match cmd {
                                                CanvasCommand::Line {
                                                    x1,
                                                    y1,
                                                    x2,
                                                    y2,
                                                    color,
                                                } => {
                                                    ctx.draw(&Line {
                                                        x1: *x1,
                                                        y1: *y1,
                                                        x2: *x2,
                                                        y2: *y2,
                                                        color: *color,
                                                    });
                                                }
                                                CanvasCommand::Circle {
                                                    x,
                                                    y,
                                                    radius,
                                                    color,
                                                } => {
                                                    ctx.draw(&Circle {
                                                        x: *x,
                                                        y: *y,
                                                        radius: *radius,
                                                        color: *color,
                                                    });
                                                }
                                                CanvasCommand::Rectangle {
                                                    x,
                                                    y,
                                                    width,
                                                    height,
                                                    color,
                                                } => {
                                                    ctx.draw(&Rectangle {
                                                        x: *x,
                                                        y: *y,
                                                        width: *width,
                                                        height: *height,
                                                        color: *color,
                                                    });
                                                }
                                                CanvasCommand::Points { points, color } => {
                                                    ctx.draw(&Points {
                                                        coords: points,
                                                        color: *color,
                                                    });
                                                }
                                            }
                                        }
                                    });

                                frame.render_widget(canvas, area);
                            }
                            Widget::TextInput {
                                x,
                                y,
                                width,
                                content,
                                cursor,
                                placeholder,
                                focused,
                                style,
                            } => {
                                let area = Rect::new(*x, *y, *width, 3);

                                let display_text = if content.is_empty() {
                                    if *focused {
                                        String::new()
                                    } else {
                                        placeholder.clone()
                                    }
                                } else {
                                    content.clone()
                                };

                                // Calculate visible window for scrolling
                                let inner_width = (width.saturating_sub(2)) as usize; // Account for borders
                                let chars: Vec<char> = display_text.chars().collect();

                                // Determine scroll offset to keep cursor visible
                                let scroll_offset = if *cursor > inner_width {
                                    *cursor - inner_width
                                } else {
                                    0
                                };

                                // Get visible portion of text
                                let visible_end = (scroll_offset + inner_width).min(chars.len());
                                let visible_text: String =
                                    chars[scroll_offset..visible_end].iter().collect();

                                // Add cursor if focused
                                let display_with_cursor = if *focused {
                                    let cursor_pos = cursor.saturating_sub(scroll_offset);
                                    let mut chars: Vec<char> = visible_text.chars().collect();
                                    if cursor_pos <= chars.len() {
                                        chars.insert(cursor_pos, '│'); // Using box drawing character
                                    }
                                    chars.iter().collect()
                                } else {
                                    visible_text
                                };

                                let border_style = if *focused {
                                    Style::default()
                                        .fg(style.border_color)
                                        .add_modifier(Modifier::BOLD)
                                } else {
                                    Style::default().fg(style.border_color)
                                };

                                let paragraph = Paragraph::new(display_with_cursor)
                                    .style(Style::default().fg(style.fg).bg(style.bg))
                                    .block(
                                        Block::default()
                                            .borders(Borders::ALL)
                                            .border_style(border_style),
                                    );

                                frame.render_widget(paragraph, area);
                            }
                        }
                    }
                });
            })?;
        }
        Ok(())
    });

    result?;
    Ok(Value::Null)
});

// Tui.draw_block(x, y, width, height, title, border_color)
native_fn!(FnTuiDrawBlock, "tui_draw_block", 6, |_evaluator, args| {
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

    let title = match &args[4] {
        Value::Str(s) => s.borrow().clone(),
        _ => String::new(),
    };

    let color = match &args[5] {
        Value::Str(s) => parse_color(&s.borrow()),
        _ => Color::White,
    };

    WIDGETS.with(|w| {
        w.borrow_mut().push(Widget::Block {
            x,
            y,
            width,
            height,
            title,
            color,
        });
    });

    Ok(Value::Null)
});

// Tui.draw_text(x, y, width, height, text, fg_color, bg_color)
native_fn!(FnTuiDrawText, "tui_draw_text", 7, |_evaluator, args| {
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

    let text = match &args[4] {
        Value::Str(s) => s.borrow().clone(),
        _ => String::new(),
    };

    let fg = match &args[5] {
        Value::Str(s) => parse_color(&s.borrow()),
        _ => Color::White,
    };

    let bg = match &args[6] {
        Value::Str(s) => parse_color(&s.borrow()),
        Value::Null => Color::Reset,
        _ => Color::Reset,
    };

    WIDGETS.with(|w| {
        w.borrow_mut().push(Widget::Text {
            x,
            y,
            width,
            height,
            text,
            fg,
            bg,
        });
    });

    Ok(Value::Null)
});

// Tui.draw_list(x, y, width, height, items, selected, color, title)
// items: List of strings, selected: index of selected item
native_fn!(FnTuiDrawList, "tui_draw_list", 8, |_evaluator, args| {
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

    let items = match &args[4] {
        Value::List(list) => list
            .borrow()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
        _ => vec![],
    };

    let selected = if let Value::Num(n) = args[5] {
        n.0 as usize
    } else {
        0
    };

    let color = match &args[6] {
        Value::Str(s) => parse_color(&s.borrow()),
        _ => Color::Cyan,
    };

    let title = match &args[7] {
        Value::Str(s) => s.borrow().clone(),
        _ => String::new(),
    };

    WIDGETS.with(|w| {
        w.borrow_mut().push(Widget::List {
            x,
            y,
            width,
            height,
            items,
            selected,
            color,
            title,
        });
    });

    Ok(Value::Null)
});

// Tui.draw_progress(x, y, width, percent, label, color)
// percent: 0-100
native_fn!(
    FnTuiDrawProgress,
    "tui_draw_progress",
    6,
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

        let percent = if let Value::Num(n) = args[3] {
            (n.0.clamp(0.0, 100.0)) as u16
        } else {
            0
        };

        let label = match &args[4] {
            Value::Str(s) => s.borrow().clone(),
            _ => String::new(),
        };

        let color = match &args[5] {
            Value::Str(s) => parse_color(&s.borrow()),
            _ => Color::Green,
        };

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::Progress {
                x,
                y,
                width,
                percent,
                label,
                color,
            });
        });

        Ok(Value::Null)
    }
);

// Helper function to parse color strings
fn parse_color(s: &str) -> Color {
    match s.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        _ => Color::White,
    }
}
