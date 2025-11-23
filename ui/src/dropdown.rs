use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, rgb,
};

use crate::Popover;

#[derive(IntoElement)]
pub struct MenuItem {
    id: ElementId,
    label: SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl MenuItem {
    pub fn new(id: ElementId, label: SharedString) -> Self {
        Self {
            id,
            label,
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for MenuItem {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        let mut root = div()
            .id(self.id)
            .rounded_md()
            .px_2()
            .py_0p5()
            .text_sm()
            .hover(|this| this.bg(rgb(0xfafafa)))
            .child(self.label);

        if let Some(click_handler) = self.on_click {
            root = root.on_click(click_handler);
        }

        root
    }
}

#[derive(IntoElement)]
pub struct Dropdown {
    id: ElementId,
    trigger: AnyElement,
    menu: Vec<MenuItem>,
    selected: Option<usize>,
    open: bool,
}

impl Dropdown {
    pub fn new(id: ElementId, trigger: AnyElement, menu: Vec<MenuItem>) -> Self {
        Self {
            id,
            trigger,
            menu,
            selected: None,
            open: false,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn selected(self) -> Option<usize> {
        self.selected
    }
}

impl RenderOnce for Dropdown {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        let menu = div().children(self.menu);

        Popover::new(self.id, self.trigger, menu.into_any_element()).open(self.open)
    }
}
