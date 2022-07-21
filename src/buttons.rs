use gtk;
use gtk::{Button, Entry};

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