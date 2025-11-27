mod canvas;
mod text_input;

use ordered_float::OrderedFloat;
use std::{cell::RefCell, collections::HashMap, io, rc::Rc};

use crate::{
    evaluator::{
        Callable, ErrKind, EvalResult, Evaluator, RuntimeEvent,
        natives::tui::{
            canvas::{CanvasWidget, FnTuiCreateCanvas, render_canvas},
            text_input::{FnTuiCreateTextInput, TextInputWidget, render_text_input},
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
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
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
        "draw_block_rect".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawBlockRect), false)),
    );
    methods.insert(
        "draw_text".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawText), false)),
    );
    methods.insert(
        "draw_text_rect".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawTextRect), false)),
    );
    methods.insert(
        "draw_list".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawList), false)),
    );
    methods.insert(
        "draw_list_rect".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawListRect), false)),
    );
    methods.insert(
        "draw_checkbox".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawCheckbox), false)),
    );
    methods.insert(
        "draw_checkbox_rect".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawCheckboxRect), false)),
    );
    methods.insert(
        "draw_progress".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawProgress), false)),
    );
    methods.insert(
        "draw_progress_rect".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiDrawProgressRect), false)),
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
    methods.insert(
        "split_row".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiSplitRow), false)),
    );
    methods.insert(
        "split_col".into(),
        Method::Native(NativeMethod::new(Rc::new(FnTuiSplitCol), false)),
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
        style: TuiStyle,
    },
    BlockRect {
        rect_id: usize,
        title: String,
        style: TuiStyle,
    },
    Text {
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        text: String,
        style: TuiStyle,
    },
    TextRect {
        rect_id: usize,
        text: String,
        style: TuiStyle,
    },
    Checkbox {
        x: u16,
        y: u16,
        label: String,
        checked: bool,
        style: TuiStyle,
    },
    CheckboxRect {
        rect_id: usize,
        label: String,
        checked: bool,
        style: TuiStyle,
    },
    List {
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        items: Vec<String>,
        selected: usize,
        style: TuiStyle,
        title: String,
    },
    ListRect {
        rect_id: usize,
        items: Vec<String>,
        selected: usize,
        style: TuiStyle,
        title: String,
    },
    Progress {
        x: u16,
        y: u16,
        width: u16,
        percent: u16,
        label: String,
        style: TuiStyle,
    },
    ProgressRect {
        rect_id: usize,
        percent: u16,
        label: String,
        style: TuiStyle,
    },
    Canvas(CanvasWidget),
    TextInput(TextInputWidget),
}

impl Widget {
    fn render(&self, frame: &mut Frame<'_>) {
        match self {
            Widget::Block {
                x,
                y,
                width,
                height,
                title,
                style,
            } => {
                let area = Rect::new(*x, *y, *width, *height);
                let block = Block::default()
                    .title(title.clone())
                    .borders(Borders::ALL)
                    .style(style.text_style())
                    .border_style(Style::default().fg(style.accent));
                frame.render_widget(block, area);
            }
            Widget::BlockRect {
                rect_id,
                title,
                style,
            } => {
                if let Some(area) = rect_from_id(*rect_id, frame) {
                    let block = Block::default()
                        .title(title.clone())
                        .borders(Borders::ALL)
                        .style(style.text_style())
                        .border_style(Style::default().fg(style.accent));
                    frame.render_widget(block, area);
                }
            }
            Widget::Text {
                x,
                y,
                width,
                height,
                text,
                style,
            } => {
                let area = Rect::new(*x, *y, *width, *height);
                let paragraph = Paragraph::new(text.clone())
                    .style(style.text_style())
                    .wrap(Wrap { trim: false });
                frame.render_widget(paragraph, area);
            }
            Widget::TextRect {
                rect_id,
                text,
                style,
            } => {
                if let Some(area) = rect_from_id(*rect_id, frame) {
                    let paragraph = Paragraph::new(text.clone())
                        .style(style.text_style())
                        .wrap(Wrap { trim: false });
                    frame.render_widget(paragraph, area);
                }
            }
            Widget::Checkbox {
                x,
                y,
                label,
                checked,
                style,
            } => {
                let text = if *checked {
                    format!("[x] {}", label)
                } else {
                    format!("[ ] {}", label)
                };
                let render_style = if *checked {
                    style.text_style().fg(style.accent)
                } else {
                    style.text_style()
                };
                let width = text.len() as u16;
                let area = widget_rect(frame, *x, *y, width, 1);
                let paragraph = Paragraph::new(text).style(render_style);
                frame.render_widget(paragraph, area);
            }
            Widget::CheckboxRect {
                rect_id,
                label,
                checked,
                style,
            } => {
                if let Some(area) = rect_from_id(*rect_id, frame) {
                    let text = if *checked {
                        format!("[x] {}", label)
                    } else {
                        format!("[ ] {}", label)
                    };
                    let render_style = if *checked {
                        style.text_style().fg(style.accent)
                    } else {
                        style.text_style()
                    };
                    let paragraph = Paragraph::new(text).style(render_style);
                    frame.render_widget(paragraph, area);
                }
            }
            Widget::List {
                x,
                y,
                width,
                height,
                items,
                selected,
                style,
                title,
            } => {
                let area = Rect::new(*x, *y, *width, *height);
                let normal = style.text_style();
                let highlight = Style::default()
                    .fg(style.accent)
                    .bg(style.bg)
                    .add_modifier(Modifier::BOLD);

                let list_items: Vec<ListItem> = items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let prefix = if i == *selected { "> " } else { "  " };
                        let item_style = if i == *selected { highlight } else { normal };
                        ListItem::new(format!("{}{}", prefix, item)).style(item_style)
                    })
                    .collect();

                let list = List::new(list_items).block(
                    Block::default()
                        .title(title.clone())
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(style.accent)),
                );

                frame.render_widget(list, area);
            }
            Widget::ListRect {
                rect_id,
                items,
                selected,
                style,
                title,
            } => {
                if let Some(area) = rect_from_id(*rect_id, frame) {
                    let normal = style.text_style();
                    let highlight = Style::default()
                        .fg(style.accent)
                        .bg(style.bg)
                        .add_modifier(Modifier::BOLD);

                    let list_items: Vec<ListItem> = items
                        .iter()
                        .enumerate()
                        .map(|(i, item)| {
                            let prefix = if i == *selected { "> " } else { "  " };
                            let item_style = if i == *selected { highlight } else { normal };
                            ListItem::new(format!("{}{}", prefix, item)).style(item_style)
                        })
                        .collect();

                    let list = List::new(list_items).block(
                        Block::default()
                            .title(title.clone())
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(style.accent)),
                    );

                    frame.render_widget(list, area);
                }
            }
            Widget::Progress {
                x,
                y,
                width,
                percent,
                label,
                style,
            } => {
                let area = Rect::new(*x, *y, *width, 3);
                let gauge = Gauge::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(style.accent)),
                    )
                    .gauge_style(style.text_style().fg(style.accent))
                    .percent(*percent)
                    .label(label.clone());
                frame.render_widget(gauge, area);
            }
            Widget::ProgressRect {
                rect_id,
                percent,
                label,
                style,
            } => {
                if let Some(area) = rect_from_id(*rect_id, frame) {
                    let gauge = Gauge::default()
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(style.accent)),
                        )
                        .gauge_style(style.text_style().fg(style.accent))
                        .percent(*percent)
                        .label(label.clone());
                    frame.render_widget(gauge, area);
                }
            }
            Widget::Canvas(widget) => render_canvas(
                frame,
                widget,
                widget_rect(frame, widget.x, widget.y, widget.width, widget.height),
            ),
            Widget::TextInput(widget) => render_text_input(
                frame,
                widget,
                widget_rect(frame, widget.x, widget.y, widget.width, 3),
            ),
        }
    }
}

pub(super) fn widget_rect(frame: &Frame<'_>, x: u16, y: u16, width: u16, height: u16) -> Rect {
    let parent = frame.area();
    let y = y.min(parent.height);
    let x = x.min(parent.width);
    let height = height.min(parent.height.saturating_sub(y));
    let width = width.min(parent.width.saturating_sub(x));

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(y),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(parent);
    let row_area = rows[1];

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(x),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(row_area)[1]
}

fn rect_from_id(id: usize, _frame: &Frame<'_>) -> Option<Rect> {
    RECTS.with(|r| r.borrow().get(id).copied())
}

fn reset_layout_state() {
    LAYOUT_CMDS.with(|c| c.borrow_mut().clear());
    NEXT_RECT_ID.with(|n| *n.borrow_mut() = 1);
    RECTS.with(|r| r.borrow_mut().clear());
}

fn compute_rects(root: Rect) {
    let mut rects = vec![Rect::new(0, 0, 0, 0); NEXT_RECT_ID.with(|n| *n.borrow())];
    rects[0] = root;

    LAYOUT_CMDS.with(|cmds| {
        for cmd in cmds.borrow().iter() {
            let splits = Layout::default()
                .direction(cmd.direction)
                .constraints(cmd.constraints.clone())
                .split(rects[cmd.parent]);
            for (i, rect) in splits.iter().enumerate() {
                if cmd.start + i < rects.len() {
                    rects[cmd.start + i] = *rect;
                }
            }
        }
    });

    RECTS.with(|r| {
        *r.borrow_mut() = rects;
    });
}

#[derive(Clone)]
pub struct TuiStyle {
    pub fg: Color,
    pub bg: Color,
    pub accent: Color,
}

impl Default for TuiStyle {
    fn default() -> Self {
        Self {
            fg: Color::White,
            bg: Color::Reset,
            accent: Color::Cyan,
        }
    }
}

impl TuiStyle {
    fn color_from_value(val: Option<&Value>, default: Color) -> Color {
        match val {
            Some(Value::Str(s)) => parse_color(&s.borrow()),
            Some(Value::Null) => Color::Reset,
            _ => default,
        }
    }

    fn with_fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    fn with_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    fn with_accent(mut self, accent: Color) -> Self {
        self.accent = accent;
        self
    }

    fn from_args(
        fg_arg: Option<&Value>,
        bg_arg: Option<&Value>,
        accent_arg: Option<&Value>,
    ) -> Self {
        Self::default()
            .with_fg(Self::color_from_value(fg_arg, Color::White))
            .with_bg(Self::color_from_value(bg_arg, Color::Reset))
            .with_accent(Self::color_from_value(accent_arg, Color::Cyan))
    }

    fn text_style(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
    }

    fn accent_style(&self) -> Style {
        Style::default().fg(self.accent)
    }

    fn border_style(&self, focused: bool) -> Style {
        let base = self.accent_style();
        if focused {
            base.add_modifier(Modifier::BOLD)
        } else {
            base
        }
    }
}

// Global terminal instance and widget buffer
thread_local! {
    static TERMINAL: RefCell<Option<Terminal<CrosstermBackend<io::Stdout>>>> = RefCell::new(None);
    static WIDGETS: RefCell<Vec<Widget>> = RefCell::new(Vec::new());
    static LAYOUT_CMDS: RefCell<Vec<LayoutCmd>> = RefCell::new(Vec::new());
    static NEXT_RECT_ID: RefCell<usize> = RefCell::new(1); // 0 is root
    static RECTS: RefCell<Vec<Rect>> = RefCell::new(Vec::new());
}

#[derive(Clone)]
struct LayoutCmd {
    parent: usize,
    constraints: Vec<Constraint>,
    direction: Direction,
    start: usize,
}

// Tui.init(): initializes the TUI (enters alternate screen, raw mode)
native_fn!(FnTuiInit, "tui_init", 0, |_evaluator, _args, _cursor| {
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
native_fn!(
    FnTuiCleanup,
    "tui_cleanup",
    0,
    |_evaluator, _args, _cursor| {
        TERMINAL.with(|t| {
            if let Some(mut terminal) = t.borrow_mut().take() {
                let _ = disable_raw_mode();
                let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
                let _ = terminal.show_cursor();
            }
        });

        Ok(Value::Null)
    }
);

// Tui.clear(): clears the widget buffer (call this at the start of each frame)
native_fn!(FnTuiClear, "tui_clear", 0, |_evaluator, _args, _cursor| {
    WIDGETS.with(|w| {
        w.borrow_mut().clear();
    });
    reset_layout_state();

    Ok(Value::Null)
});

// Tui.render(): renders all accumulated widgets to the screen
native_fn!(
    FnTuiRender,
    "tui_render",
    0,
    |_evaluator, _args, _cursor| {
        let result = TERMINAL.with(|t| -> io::Result<()> {
            if let Some(terminal) = t.borrow_mut().as_mut() {
                terminal.draw(|frame| {
                    compute_rects(frame.area());
                    WIDGETS.with(|w| {
                        for widget in w.borrow().iter() {
                            widget.render(frame);
                        }
                    });
                })?;
            }
            Ok(())
        });

        result?;
        Ok(Value::Null)
    }
);

// Tui.draw_block(x, y, width, height, title, border_color)
native_fn!(
    FnTuiDrawBlock,
    "tui_draw_block",
    6,
    |_evaluator, args, cursor| {
        let x = args[0].check_num(cursor, Some("x position".into()))? as u16;
        let y = args[1].check_num(cursor, Some("y position".into()))? as u16;
        let width = args[2].check_num(cursor, Some("width".into()))? as u16;
        let height = args[3].check_num(cursor, Some("height".into()))? as u16;

        let title = string_from_value(&args[4]);
        let style = TuiStyle::from_args(None, None, args.get(5));

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::Block {
                x,
                y,
                width,
                height,
                title,
                style,
            });
        });

        Ok(Value::Null)
    }
);

// Tui.draw_block_rect(rect_id, title, border_color)
native_fn!(
    FnTuiDrawBlockRect,
    "tui_draw_block_rect",
    3,
    |_evaluator, args, cursor| {
        let rect_id = args[0].check_num(cursor, Some("rect id".into()))? as usize;
        let title = string_from_value(&args[1]);
        let style = TuiStyle::from_args(None, None, args.get(2));

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::BlockRect {
                rect_id,
                title,
                style,
            });
        });

        Ok(Value::Null)
    }
);

// Tui.draw_text(x, y, width, height, text, fg_color, bg_color)
native_fn!(
    FnTuiDrawText,
    "tui_draw_text",
    7,
    |_evaluator, args, cursor| {
        let x = args[0].check_num(cursor, Some("x position".into()))? as u16;
        let y = args[1].check_num(cursor, Some("y position".into()))? as u16;
        let width = args[2].check_num(cursor, Some("width".into()))? as u16;
        let height = args[3].check_num(cursor, Some("height".into()))? as u16;

        let text = string_from_value(&args[4]);
        let style = TuiStyle::from_args(args.get(5), args.get(6), None);

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::Text {
                x,
                y,
                width,
                height,
                text,
                style,
            });
        });

        Ok(Value::Null)
    }
);

// Tui.draw_text_rect(rect_id, text, fg_color, bg_color)
native_fn!(
    FnTuiDrawTextRect,
    "tui_draw_text_rect",
    4,
    |_evaluator, args, cursor| {
        let rect_id = args[0].check_num(cursor, Some("rect id".into()))? as usize;
        let text = string_from_value(&args[1]);
        let style = TuiStyle::from_args(args.get(2), args.get(3), None);

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::TextRect {
                rect_id,
                text,
                style,
            });
        });

        Ok(Value::Null)
    }
);

// Tui.draw_list(x, y, width, height, items, selected, color, title)
// items: List of strings, selected: index of selected item
native_fn!(
    FnTuiDrawList,
    "tui_draw_list",
    8,
    |_evaluator, args, cursor| {
        let x = args[0].check_num(cursor, Some("x".into()))? as u16;
        let y = args[1].check_num(cursor, Some("y".into()))? as u16;
        let width = args[2].check_num(cursor, Some("width".into()))? as u16;
        let height = args[3].check_num(cursor, Some("height".into()))? as u16;

        let items = match &args[4] {
            Value::List(list) => list
                .borrow()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            _ => vec![],
        };

        let selected_val = args[5].check_num(cursor, Some("selected index".into()))?;
        let selected = if selected_val < 0.0 {
            0
        } else {
            selected_val as usize
        };

        let style = TuiStyle::from_args(None, None, args.get(6));
        let title = string_from_value(&args[7]);

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::List {
                x,
                y,
                width,
                height,
                items,
                selected,
                style,
                title,
            });
        });

        Ok(Value::Null)
    }
);

// Tui.draw_checkbox(x, y, label, checked, fg_color, bg_color, accent_color)
native_fn!(
    FnTuiDrawCheckbox,
    "tui_draw_checkbox",
    7,
    |_evaluator, args, cursor| {
        let x = args[0].check_num(cursor, Some("x position".into()))? as u16;
        let y = args[1].check_num(cursor, Some("y position".into()))? as u16;
        let label = string_from_value(&args[2]);
        let checked = args[3].check_bool(cursor, Some("checked".into()))?;

        let style = TuiStyle::from_args(args.get(4), args.get(5), args.get(6));

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::Checkbox {
                x,
                y,
                label,
                checked,
                style,
            });
        });

        Ok(Value::Null)
    }
);

// Tui.draw_progress(x, y, width, percent, label, color)
// percent: 0-100
native_fn!(
    FnTuiDrawProgress,
    "tui_draw_progress",
    6,
    |_evaluator, args, cursor| {
        let x = args[0].check_num(cursor, Some("x".into()))? as u16;
        let y = args[1].check_num(cursor, Some("y".into()))? as u16;
        let width = args[2].check_num(cursor, Some("width".into()))? as u16;
        let percent = args[3]
            .check_num(cursor, Some("percent".into()))?
            .clamp(0.0, 100.0) as u16;

        let label = string_from_value(&args[4]);
        let style = TuiStyle::from_args(None, None, args.get(5));

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::Progress {
                x,
                y,
                width,
                percent,
                label,
                style,
            });
        });

        Ok(Value::Null)
    }
);

// Tui.draw_list_rect(rect_id, items, selected, color, title)
native_fn!(
    FnTuiDrawListRect,
    "tui_draw_list_rect",
    5,
    |_evaluator, args, cursor| {
        let rect_id = args[0].check_num(cursor, Some("rect id".into()))? as usize;

        let items = match &args[1] {
            Value::List(list) => list
                .borrow()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            _ => vec![],
        };
        let selected_val = args[2].check_num(cursor, Some("selected index".into()))?;
        let selected = if selected_val < 0.0 {
            0
        } else {
            selected_val as usize
        };

        let style = TuiStyle::from_args(None, None, args.get(3));
        let title = string_from_value(&args[4]);

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::ListRect {
                rect_id,
                items,
                selected,
                style,
                title,
            });
        });

        Ok(Value::Null)
    }
);

// Tui.draw_progress_rect(rect_id, percent, label, color)
native_fn!(
    FnTuiDrawProgressRect,
    "tui_draw_progress_rect",
    4,
    |_evaluator, args, cursor| {
        let rect_id = args[0].check_num(cursor, Some("rect id".into()))? as usize;
        let percent = args[1]
            .check_num(cursor, Some("percent".into()))?
            .clamp(0.0, 100.0) as u16;
        let label = string_from_value(&args[2]);
        let style = TuiStyle::from_args(None, None, args.get(3));

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::ProgressRect {
                rect_id,
                percent,
                label,
                style,
            });
        });

        Ok(Value::Null)
    }
);

// Tui.draw_checkbox_rect(rect_id, label, checked, fg, bg, accent)
native_fn!(
    FnTuiDrawCheckboxRect,
    "tui_draw_checkbox_rect",
    6,
    |_evaluator, args, cursor| {
        let rect_id = args[0].check_num(cursor, Some("rect id".into()))? as usize;
        let label = string_from_value(&args[1]);
        let checked = args[2].check_bool(cursor, Some("checked".into()))?;
        let style = TuiStyle::from_args(args.get(3), args.get(4), args.get(5));

        WIDGETS.with(|w| {
            w.borrow_mut().push(Widget::CheckboxRect {
                rect_id,
                label,
                checked,
                style,
            });
        });

        Ok(Value::Null)
    }
);

// Split utilities: percent-only constraints for simplicity
fn constraints_from_value(
    val: &Value,
    cursor: crate::lexer::cursor::Cursor,
) -> EvalResult<Vec<Constraint>> {
    if let Value::List(list) = val {
        let mut out = Vec::new();
        for v in list.borrow().iter() {
            let p = v
                .check_num(cursor, Some("constraint".into()))?
                .clamp(0.0, 100.0);
            out.push(Constraint::Percentage(p as u16));
        }
        Ok(out)
    } else {
        Err(RuntimeEvent::error(
            ErrKind::Type,
            "constraints must be a List of numbers (percentages)".into(),
            cursor,
        ))
    }
}

// Tui.split_row(parent_rect_id, constraints:list<num>) -> list<num rect_ids>
native_fn!(
    FnTuiSplitRow,
    "tui_split_row",
    2,
    |_evaluator, args, cursor| {
        let parent = args[0].check_num(cursor, Some("parent rect id".into()))? as usize;
        let constraints = constraints_from_value(&args[1], cursor)?;
        let count = constraints.len();
        let start = NEXT_RECT_ID.with(|n| {
            let start = *n.borrow();
            *n.borrow_mut() += count;
            start
        });

        LAYOUT_CMDS.with(|cmds| {
            cmds.borrow_mut().push(LayoutCmd {
                parent,
                constraints: constraints.clone(),
                direction: Direction::Horizontal,
                start,
            });
        });

        let rect_ids: Vec<Value> = (start..start + count)
            .map(|id| Value::Num(OrderedFloat(id as f64)))
            .collect();
        Ok(Value::List(Rc::new(RefCell::new(rect_ids))))
    }
);

// Tui.split_col(parent_rect_id, constraints:list<num>) -> list<num rect_ids>
native_fn!(
    FnTuiSplitCol,
    "tui_split_col",
    2,
    |_evaluator, args, cursor| {
        let parent = args[0].check_num(cursor, Some("parent rect id".into()))? as usize;
        let constraints = constraints_from_value(&args[1], cursor)?;
        let count = constraints.len();
        let start = NEXT_RECT_ID.with(|n| {
            let start = *n.borrow();
            *n.borrow_mut() += count;
            start
        });

        LAYOUT_CMDS.with(|cmds| {
            cmds.borrow_mut().push(LayoutCmd {
                parent,
                constraints: constraints.clone(),
                direction: Direction::Vertical,
                start,
            });
        });

        let rect_ids: Vec<Value> = (start..start + count)
            .map(|id| Value::Num(OrderedFloat(id as f64)))
            .collect();
        Ok(Value::List(Rc::new(RefCell::new(rect_ids))))
    }
);

// Helper function to parse color strings
pub fn parse_color(s: &str) -> Color {
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

fn string_from_value(value: &Value) -> String {
    match value {
        Value::Str(s) => s.borrow().clone(),
        _ => String::new(),
    }
}
