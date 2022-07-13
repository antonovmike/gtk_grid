use glib::clone;
// use gtk::glib;
use gtk::prelude::*;
use std::ops::Add;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.grid-packing"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    // Create a new window, set its title and default size
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Grid Packing");
    window.set_default_size(200, 120);

    // Construct the grid that is going contain our buttons
    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    // Add the grid in the window
    window.set_child(Some(&grid));

    // let padding_between_children = 0;
    // Box::new() expected 1 argument!!!
    // let horizontal_box = Box::new(Orientation::Horizontal, padding_between_children);
    // let vertical_box = Box::new(Orientation::Vertical, padding_between_children);

    // Create the first button and put it into the grid at (0, 0)
    let button_1 = gtk::Button::with_label("Button 1");
    button_1.connect_clicked(move |_| calculate("1"));
    grid.attach(&button_1, 0, 0, 1, 1);

    // Create the second button and put it into the grid at (1, 0)
    let button_2 = gtk::Button::with_label("Button 2");
    button_2.connect_clicked(move |_| calculate("+"));
    grid.attach(&button_2, 1, 0, 1, 1);

    let button_3 = gtk::Button::with_label("Button 3");
    button_3.connect_clicked(move |_| calculate("2"));
    grid.attach(&button_3, 0, 1, 2, 1);

    // let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app
    
    let button_4 = gtk::Button::with_label("Button 4");
    button_4.connect_clicked(move |_| {
        println!("Hello World 4");
        calculate("=")
    });
    grid.attach(&button_4, 0, 2, 2, 1);

    // Create the quit button and put it into the grid at (0, 1)
    let quit_button = gtk::Button::with_label("Quit");
    quit_button.connect_clicked(clone!(@weak window => move |_| 
        unsafe {
            window.destroy()
        }
    ));
    grid.attach(&quit_button, 2, 0, 1, 3);

    window.show_all();
}

fn calculate(input: &str) {
    let first = if input == "1" {1} else {1};
    // let plus = "";
    let second = if input == "2" {2} else {2};
    let result = match input {
        "=" => first + second,
        _ => 0,
    };
    println!("calculate: {}", result);
}