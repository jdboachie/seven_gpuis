use gpui::{Div, FontWeight, SharedString, Stateful, Styled, div, rgb, prelude::*};

pub fn button(text: &str) -> Stateful<Div> {
    div()
        .id(SharedString::from(text.to_string()))
        .flex_none()
        .px_4()
        .py_1()
        .h_9()
        .items_center()
        .justify_center()
        .bg(rgb(0xffffffff))
        .active(|this| this.opacity(0.90))
        .cursor_pointer()
        .border_1()
        .border_color(rgb(0xe0e0e0))
        .rounded_md()
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer()
        .child(text.to_string())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
