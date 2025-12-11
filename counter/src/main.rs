use gpui::{
    AppContext, Application, Bounds, Render, Styled, WindowBounds, WindowOptions, div,
    prelude::*, px, size,
};
use ui::{Button, Theme, ThemeAble};

struct Counter {
    count: u32,
}

impl Counter {
    fn new(count: u32) -> Self {
        Self { count }
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

impl Render for Counter {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let theme = cx.theme();

        div()
            .p_4()
            .flex()
            .gap_4()
            .size_full()
            .justify_center()
            .items_center()
            .bg(theme.ground)
            .gap_4()
            .child(
                div()
                    .text_2xl()
                    .child(format!("{}", self.count)),
            )
            .child(
                Button::new("b".into())
                    .with_label("Count".into())
                    .on_click(cx.listener(|this, _, _, _| {
                        this.increment();
                    })),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        Theme::init(cx);
        let bounds = Bounds::centered(None, size(px(200.0), px(125.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Counter::new(0)),
        )
        .unwrap();

        cx.activate(true);
    });
}
