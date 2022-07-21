use gtk;
use gtk::{Button, Entry, Label};

pub fn create_button(label: &'static str) -> Button {
    let margin = 2;
    Button::builder()
        .label(label)
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build()
}

pub fn create_entry() -> Entry {
    let margin = 2;
    Entry::builder()
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build()
}

pub fn create_label(label: Option<&str>) -> Label {
    let margin = 2;
    let text: &str = label.unwrap_or("default string");
    Label::builder()
        .label(text)
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build()
}