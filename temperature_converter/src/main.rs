use gpui::{
    App, Application, Bounds, Context, Entity, EventEmitter, FocusHandle, Focusable, KeyBinding,
    SharedString, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use ui::{
    Backspace, Copy, Cut, Delete, End, Home, Left, Paste, Quit, Right, SelectAll, SelectLeft,
    SelectRight, ShowCharacterPalette, TextInput,
};

struct ConversionModel {
    c_val: f32,
    f_val: f32,
}

impl ConversionModel {
    fn new() -> Self {
        ConversionModel {
            c_val: 0.,
            f_val: 32.,
        }
    }

    fn c_to_f(c: f32) -> f32 {
        (c * 1.8) + 32.
    }

    fn f_to_c(f: f32) -> f32 {
        (f - 32.) * (5. / 9.)
    }
}

struct TemperatureConverterApp {
    model: Entity<ConversionModel>,
    celcius_input: Entity<TextInput>,
    fahrenheit_input: Entity<TextInput>,
    focus_handle: FocusHandle,
}

impl Focusable for TemperatureConverterApp {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TemperatureConverterApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0xf8f8f8))
            .track_focus(&self.focus_handle(cx))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .child(
                div()
                    .size_full()
                    .p_2()
                    .gap_2()
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .w_full()
                            .p_2()
                            .rounded_md()
                            .child(self.celcius_input.clone())
                            .on_key_up(cx.listener(|app, _event, _window, cx| {
                                app.model.update(cx, |this, cx| {
                                    this.c_val = match app.celcius_input.read(cx).content.parse() {
                                        Ok(val) => val,
                                        Err(_) => {
                                            return;
                                        }
                                    };
                                    cx.emit(ChangeEvent {
                                        c_changed: true,
                                        f_changed: false,
                                    });
                                    // cx.notify();
                                });
                            })),
                    )
                    .child("Celcius = ")
                    .child(
                        div()
                            .w_full()
                            .p_2()
                            .rounded_md()
                            .child(self.fahrenheit_input.clone())
                            .on_key_up(cx.listener(|app, _, _, cx| {
                                app.model.update(cx, |this, cx| {
                                    this.f_val = match app.fahrenheit_input.read(cx).content.parse()
                                    {
                                        Ok(val) => val,
                                        Err(_) => {
                                            return;
                                        }
                                    };
                                    cx.emit(ChangeEvent {
                                        c_changed: false,
                                        f_changed: true,
                                    });
                                    // cx.notify();
                                });
                            })),
                    )
                    .child(" Fahrenheit"),
            )
    }
}

struct ChangeEvent {
    c_changed: bool,
    f_changed: bool,
}
impl EventEmitter<ChangeEvent> for ConversionModel {}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("ctrl-a", SelectAll, None),
            KeyBinding::new("ctrl-v", Paste, None),
            KeyBinding::new("ctrl-c", Copy, None),
            KeyBinding::new("ctrl-x", Cut, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("ctrl-space", ShowCharacterPalette, None),
        ]);

        let bounds = Bounds::centered(None, size(px(500.0), px(100.0)), cx);
        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    let model = cx.new(|_| ConversionModel::new());
                    let c_input = cx.new(|cx| {
                        cx.subscribe(
                            &model,
                            |this: &mut TextInput, model: Entity<ConversionModel>, event, cx| {
                                if event.f_changed {
                                    this.content = SharedString::from(
                                        ConversionModel::f_to_c(model.read(cx).f_val).to_string(),
                                    );
                                };
                                cx.notify();
                            },
                        )
                        .detach();
                        TextInput {
                            focus_handle: cx.focus_handle(),
                            content: model.read(cx).c_val.to_string().into(),
                            placeholder: "Type here...".into(),
                            selected_range: 0..0,
                            selection_reversed: false,
                            marked_range: None,
                            last_layout: None,
                            last_bounds: None,
                            is_selecting: false,
                        }
                    });
                    let f_input = cx.new(|cx| {
                        cx.subscribe(
                            &model,
                            |this: &mut TextInput, model: Entity<ConversionModel>, event, cx| {
                                if event.c_changed {
                                    this.content = SharedString::from(
                                        ConversionModel::c_to_f(model.read(cx).c_val).to_string(),
                                    );
                                };
                                cx.notify();
                            },
                        )
                        .detach();

                        TextInput {
                            focus_handle: cx.focus_handle(),
                            content: model.read(cx).f_val.to_string().into(),
                            placeholder: "Type here...".into(),
                            selected_range: 0..0,
                            selection_reversed: false,
                            marked_range: None,
                            last_layout: None,
                            last_bounds: None,
                            is_selecting: false,
                        }
                    });

                    cx.new(|cx| TemperatureConverterApp {
                        celcius_input: c_input,
                        fahrenheit_input: f_input,
                        focus_handle: cx.focus_handle(),
                        model: model,
                    })
                },
            )
            .unwrap();

        window
            .update(cx, |view, window, cx| {
                window.focus(&view.celcius_input.focus_handle(cx));
                cx.activate(true);
            })
            .unwrap();
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("ctrl+q", Quit, None),
        ]);
    });
}
