use gpui::{App, Global, Hsla, hsla, rgb, rgba};

pub trait ThemeAble {
    fn theme(&self) -> &Theme;
}

impl ThemeAble for App {
    fn theme(&self) -> &Theme {
        self.global::<Theme>()
    }
}

pub struct Theme {
    pub border: Hsla,
    pub button_surface: Hsla,
    pub foreground: Hsla,
    pub ground: Hsla,
    pub highlight: Hsla,
    pub primary: Hsla,
    pub primary_hover: Hsla,
    pub surface: Hsla,
    pub transparent: Hsla,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: rgba(0x0078D4FF).into(),
            primary_hover: rgba(0x0078D4DD).into(),
            foreground: rgb(0x2f2f2f).into(),
            border: rgba(0xd3d3d6bb).into(),
            ground: rgb(0xF3F4F6).into(),
            surface: rgb(0xFFFFFF).into(),
            button_surface: rgb(0xFFFFFF).into(),
            transparent: hsla(0., 0., 0., 0.),
            highlight: rgb(0xFAFAFA).into(),
        }
    }
}

impl Global for Theme {}

impl Theme {
    pub fn init(cx: &mut App) {
        cx.set_global(Theme::default());
    }
}
