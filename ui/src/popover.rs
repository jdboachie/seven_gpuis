use crate::theme::ThemeAble;
use gpui::{
    AnyElement, Corner, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    Styled, anchored, deferred, div, px,
};

const PRIORITY: usize = 10;

#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    anchor: Corner,
    trigger: AnyElement,
    content: AnyElement,
    open: bool,
}

impl Popover {
    pub fn new(id: ElementId, trigger: AnyElement, content: AnyElement) -> Self {
        Self {
            id,
            trigger,
            content,
            open: false,
            anchor: Corner::TopLeft,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn anchor(mut self, anchor: Corner) -> Self {
        self.anchor = anchor;
        self
    }
}

impl RenderOnce for Popover {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let theme = cx.theme();
        let mut el = div().id(self.id).child(self.trigger).w_full();

        if self.open {
            el = el.child(
                deferred(
                    anchored()
                        .anchor(self.anchor)
                        .snap_to_window_with_margin(px(8.0))
                        .child(
                            div()
                                .w_full()
                                .mt_0p5()
                                .border_1()
                                .border_color(theme.border)
                                .shadow_xs()
                                .rounded_md()
                                .bg(theme.surface)
                                .p_1()
                                .child(self.content),
                        ),
                )
                .with_priority(PRIORITY),
            );
        }

        el
    }
}
