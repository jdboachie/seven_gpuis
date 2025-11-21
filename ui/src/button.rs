use gpui::{
    App, ClickEvent, CursorStyle, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*, rgb, svg,
};

use crate::theme::use_theme;

pub enum ButtonVariant {
    Ghost,
    Outlined,
    Solid,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Solid
    }
}

#[derive(Default, IntoElement)]
pub struct Button {
    id: SharedString,
    disabled: bool,
    variant: ButtonVariant,
    label: Option<SharedString>,
    icon_path: Option<SharedString>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    full_width: bool,
}

impl Button {
    pub fn new(id: SharedString) -> Self {
        Button {
            id,
            disabled: false,
            variant: ButtonVariant::default(),
            label: None,
            icon_path: None,
            on_click: None,
            full_width: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_label(mut self, label: SharedString) -> Self {
        self.label = Some(label);
        self
    }

    // add icon size
    pub fn with_icon(mut self, path: SharedString) -> Self {
        self.icon_path = Some(path);
        self
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let theme = use_theme(cx);

        let mut root = div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .gap_4()
            .text_center()
            .h_8()
            .px_3()
            .rounded_md()
            .when(self.full_width, |this| this.w_full())
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| this.on_click(on_click),
            )
            .shadow_2xs()
            .text_sm()
            .text_color(match self.variant {
                ButtonVariant::Solid => rgb(0xffffff).into(),
                _ => theme.foreground,
            })
            .border_1()
            .when(self.disabled == false, |this| {
                this.active(|this| this.shadow_none())
                    .hover(|this| {
                        this.bg(match self.variant {
                            ButtonVariant::Solid => theme.primary_hover,
                            _ => theme.highlight,
                        })
                    })
            })
            .border_color(match self.variant {
                ButtonVariant::Ghost => theme.transparent,
                ButtonVariant::Outlined => theme.border,
                ButtonVariant::Solid => theme.primary_hover,
            })
            .bg(match self.variant {
                ButtonVariant::Ghost => theme.transparent,
                ButtonVariant::Outlined => theme.button_surface,
                ButtonVariant::Solid => theme.primary,
            })
            .when(self.disabled, |this| {
                this.cursor(CursorStyle::OperationNotAllowed).opacity(0.6)
            });

        if let Some(label) = self.label {
            root = root.child(label);
        }

        if let Some(icon) = self.icon_path {
            root = root
                .justify_between()
                .child(svg().path(icon).size_3().text_color(rgb(0x555555)))
        }

        root
    }
}
