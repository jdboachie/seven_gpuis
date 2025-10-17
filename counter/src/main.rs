use gpui::{
    AppContext, Application, Bounds, FontWeight, Render, Styled,
    WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use ui::button;

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
            .child(button("Increment").on_click(cx.listener(|this, _, _, _| {
                this.model.increment();
            })))
    }
}

fn main() {
    Application::new().run(|cx| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        let counter = CounterModel { count: 0 };
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| CounterApp { model: counter }),
        )
        .unwrap();

        cx.activate(true);
    });
}
