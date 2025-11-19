use gpui::{
    AppContext, Application, Bounds, FontWeight, Render, Styled, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use ui::{Button, Theme};

struct CounterModel {
    count: u32,
}

impl CounterModel {
    fn increment(&mut self) {
        self.count += 1;
    }
}

struct CounterApp {
    model: CounterModel,
}

impl CounterApp {
    fn new(model: CounterModel) -> Self {
        Self { model }
    }
}

impl Render for CounterApp {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .p_4()
            .flex()
            .flex_col()
            .gap_4()
            .size_full()
            .justify_center()
            .items_center()
            .bg(rgb(0xf7f7f7))
            .gap_2()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(format!("{}", self.model.count)),
            )
            .child(
                Button::new("b".into())
                    .with_label("Increment".into())
                    .on_click(cx.listener(|this, _, _, _| {
                        this.model.increment();
                    })),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        Theme::init(cx);

        let bounds = Bounds::centered(None, size(px(200.0), px(150.0)), cx);
        let counter = CounterModel { count: 0 };
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| CounterApp::new(counter)),
        )
        .unwrap();

        cx.activate(true);
    });
}
