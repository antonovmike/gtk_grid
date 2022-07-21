use crate::buttons::create_label;
use crate::buttons::create_entry;
use crate::buttons::create_button;
use glib::clone;
use gtk::prelude::*;
use chrono::Local;

pub fn build_ui(application: &gtk::Application) {
    // Create a new window, set it's title and default size
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("GTK grid");
    window.set_default_size(200, 120);

    // Construct the grid that is going contain buttons
    let margin = 6;
    let grid = gtk::Grid::builder()
        .margin_start(margin)
        .margin_end(margin)
        .margin_top(margin)
        .margin_bottom(margin)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(margin)
        .column_spacing(margin)
        .build();

    // Add the grid in the window
    window.set_child(Some(&grid));

    // Create DISPLAY and ENTRY
    // single line text entry widget
    let entry = create_entry();
    grid.attach(&entry, 0, 0, 4 ,1);

// --> KEYBOARD STARTS HERE <--
    // Create button and put it into the grid at

    // --> NUMERIC BUTTONS
    let button_1 = create_button("Button 1");
    let button_2 = create_button("Button 2");
    let button_3 = create_button("Button 3");
    let button_4 = create_button("Button 4");
    let button_5 = create_button("Button 5");
    let button_6 = create_button("Button 6");
    let button_7 = create_button("Button 7");
    let button_8 = create_button("Button 8");
    let button_9 = create_button("Button 9");
    let button_0 = create_button("Button 0");

    // --> BUTTON FUNCTION
    button_1.connect_clicked(move |_| println!("Button 1"));
    button_2.connect_clicked(move |_| println!("Button 2"));
    button_3.connect_clicked(move |_| println!("Button 3"));
    button_4.connect_clicked(move |_| println!("Button 4"));
    button_5.connect_clicked(move |_| println!("Button 5"));
    button_6.connect_clicked(move |_| println!("Button 6"));
    button_7.connect_clicked(move |_| println!("Button 7"));
    button_8.connect_clicked(move |_| println!("Button 8"));
    button_9.connect_clicked(move |_| println!("Button 9"));
    button_0.connect_clicked(move |_| {
        let i: u8 = rand::random();
        println!("Button 0: {}", i)
    });
  
    // --> EXTRA BUTTONS
    let counter_label = create_label(Some("0.0"));
    let plus_button = create_button("+");
    let minus_button = create_button("-");
    let time = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    let quit_button = create_button("Quit");
    let label_time = create_label(None);
    label_time.set_text(&time);

    // --> EXTRA FUNCTIONS
    plus_button.connect_clicked(glib::clone!(@weak counter_label => move |_| {
        let nb = counter_label.text()
            .parse()
            .unwrap_or(0.0);
        counter_label.set_text(&format!("{}", nb + 1.1));
    }));
    minus_button.connect_clicked(glib::clone!(@weak counter_label => move |_| {
        let nb = counter_label.text()
            .parse()
            .unwrap_or(0.0);
        counter_label.set_text(&format!("{}", nb - 1.2));
    }));
    quit_button.connect_clicked(clone!(@weak window => move |_| 
        unsafe {
            window.destroy()
        }
    ));

    //         (                h, v, w, h)
    grid.attach(&button_1,      0, 1, 1, 1);
    grid.attach(&button_2,      1, 1, 1, 1);
    grid.attach(&button_3,      2, 1, 1, 1);
    grid.attach(&button_4,      0, 2, 1, 1);
    grid.attach(&button_5,      1, 2, 1, 1);
    grid.attach(&button_6,      2, 2, 1, 1);
    grid.attach(&button_7,      0, 3, 1, 1);
    grid.attach(&button_8,      1, 3, 1, 1);
    grid.attach(&button_9,      2, 3, 1, 1);
    grid.attach(&button_0,      1, 4, 1, 1);
    grid.attach(&counter_label, 0, 5, 4, 1);
    grid.attach(&plus_button,   0, 4, 1, 1);
    grid.attach(&minus_button,  2, 4, 1, 1);
    grid.attach(&label_time,    0, 6, 4, 1); // Borrowed here
    grid.attach(&quit_button,   3, 1, 1, 4);

    // Changing time
    let tick = move || {
        let time = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        label_time.set_text(&time); // Moved here
        glib::Continue(true)
    };
    // First digit is the rate of time update in seconds
    glib::timeout_add_seconds_local(1, tick);

// --> KEYBOARD ENDS HERE <--

    window.show_all();
}