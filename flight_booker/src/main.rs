mod assets;

use assets::Assets;
use gpui::{prelude::*, *};
use ui::{
    Backspace, Button, ButtonVariant, Copy, Cut, Delete, Dropdown, End, Home, Left, MenuItem,
    Paste, Right, SelectAll, SelectLeft, SelectRight, TextInput, Theme, use_theme,
};

fn parse_date(date: SharedString) -> bool {
    let parts: Vec<&str> = date.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    let day: u16 = match parts[0].parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let month: u16 = match parts[1].parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let year: u16 = match parts[2].parse() {
        Ok(v) => v,
        Err(_) => return false,
    };

    if year < 2025 || year > 9999 {
        return false;
    }
    if month == 0 || month > 12 {
        return false;
    }
    if day == 0 {
        return false;
    }

    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 28,
        _ => return false,
    };

    day <= max_day
}

#[derive(PartialEq)]
enum FlightType {
    OneWayFlight,
    ReturnFlight,
}

struct MainWindow {
    flight_type: FlightType,
    dropdown_open: bool,
    start_input: Entity<TextInput>,
    return_input: Entity<TextInput>,
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let start_input_content = self.start_input.read(cx).content.clone();
        let return_input_content = self.return_input.read(cx).content.clone();

        let start_ok = start_input_content.is_empty() || parse_date(start_input_content.clone());
        let return_ok = return_input_content.is_empty() || parse_date(return_input_content.clone());

        let should_disable = self.flight_type == FlightType::OneWayFlight;
        self.return_input.update(cx, |this, _cx| {
            this.disabled(should_disable);
        });

        let theme = use_theme(cx);

        let dropdown_trigger = Button::new("dropdown-trigger".into())
            .variant(ButtonVariant::Outlined)
            .with_label(match self.flight_type {
                FlightType::OneWayFlight => "One way flight".into(),
                FlightType::ReturnFlight => "Return flight".into(),
            })
            .with_icon("icons/caret_down.svg".into())
            .on_click(cx.listener(|this, _event, _window, cx| {
                this.dropdown_open = !this.dropdown_open;
                cx.notify();
            }));

        let menu = vec![
            MenuItem::new("1".into(), "One-way flight".into()).on_click(cx.listener(
                |this, _event, _window, cx| {
                    this.dropdown_open = !this.dropdown_open;
                    this.flight_type = FlightType::OneWayFlight;
                    cx.stop_propagation();
                    cx.notify();
                },
            )),
            MenuItem::new("2".into(), "Return flight".into()).on_click(cx.listener(
                |this, _event, _window, cx| {
                    this.dropdown_open = !this.dropdown_open;
                    this.flight_type = FlightType::ReturnFlight;
                    cx.stop_propagation();
                    cx.notify();
                },
            )),
        ];

        div()
            .id("main-window")
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .bg(theme.ground)
            .size_full()
            .justify_start()
            .items_start()
            .text_color(theme.foreground)
            .child(
                Dropdown::new(
                    "dropdown-id".into(),
                    dropdown_trigger.into_any_element(),
                    menu,
                )
                .open(self.dropdown_open),
            )
            .child(
                div()
                    .w_full()
                    .when(!start_ok, |this| this.bg(red()))
                    .child(self.start_input.clone()),
            )
            .child(
                div()
                    .w_full()
                    .when(!return_ok, |this| this.bg(red()))
                    .child(self.return_input.clone()),
            )
            .child(
                Button::new("book-flight".into())
                    .full_width(true)
                    .with_label("Book".into())
                    .disabled(
                        start_input_content.is_empty()
                            || (self.flight_type == FlightType::ReturnFlight
                                && !parse_date(return_input_content.clone()))
                            || !start_ok,
                    )
                    .on_click(cx.listener(move |this, _event, window, cx| {
                        let msg = match this.flight_type {
                            FlightType::OneWayFlight => format!(
                                "You have booked a one-way flight on {}.",
                                start_input_content
                            ),
                            FlightType::ReturnFlight => format!(
                                "You have booked a return flight on {}, and will return on {}",
                                start_input_content, return_input_content
                            ),
                        };

                        let _ = window.prompt(PromptLevel::Info, &msg, None, &["Ok", "Cancel"], cx);
                    })),
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: "assets".into(),
        })
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(200.), px(160.0)), cx);

            Theme::init(cx);

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
            ]);

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Flight Booker".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| MainWindow {
                        flight_type: FlightType::OneWayFlight,
                        dropdown_open: false,
                        start_input: cx
                            .new(|cx| TextInput::new(cx).placeholder("DD.MM.YYYY".into())),
                        return_input: cx
                            .new(|cx| TextInput::new(cx).placeholder("DD.MM.YYYY".into())),
                    })
                },
            )
            .unwrap();
        });
}
