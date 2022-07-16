// extern crate gdk;
use gtk::Label;
use glib::clone;
use gtk::prelude::*;
use gtk::Entry;
use gdk;
use chrono::Local;

pub fn build_ui(application: &gtk::Application) {
    // Create a new window, set it's title and default size
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("GTK grid");
    window.set_default_size(200, 120);
    
    // --> listen for keyboard events
    window.connect("key_press_event", false, |values| {
        // "values" is a 2-long slice of glib::value::Value, which wrap G-types
        // You can unwrap them if you know what type they are going to be ahead of time
        // values[0] is the window and values[1] is the event
        let raw_event = &values[1].get::<gdk::Event>().unwrap();
        // You have to cast to the correct event type to access some of the fields
        match raw_event.downcast_ref::<gdk::EventKey>() {
            Some(event) => {
                // get_state() FIX IT
                // println!("key value: {:?}", std::char::from_u32(event.get_keyval()));
                // println!("modifiers: {:?}", event.get_state());
                println!("modifiers: {:?}", event);
            },
            None => {},
        }

        // You need to return Some() of a glib Value wrapping a bool
        // associated item not found in `glib::Type`rustc(E0599)
        // let result = glib::value::Value::from_type(glib::types::Type::Bool);
        let result = glib::value::Value::from_type(glib::types::Type::BOOL);
        // I can't figure out how to actually set the value of result
        // Luckally returning false is good enough for now.
        Some(result)
    });
    // .unwrap();
    // --> listen for keyboard events

    let margin = 6;

    // Construct the grid that is going contain buttons
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

    // TRY LATER
    // ??? Is it possible to attach Box to Grid?
    // !!! the trait bound `std::boxed::Box<{integer}>: glib::IsA<gtk::Widget>` is not satisfied
    // let padding_between_children = 0;
    // !!! Box::new() expected 1 argument!!!
    // let horizontal_box = Box::new(Orientation::Horizontal, padding_between_children);
    // let vertical_box = Box::new(Orientation::Vertical, padding_between_children);
    // !!! So this is works
    // let horizontal_box = Box::new(padding_between_children);
    // let vertical_box = Box::new(padding_between_children);
    // grid.attach(&horizontal_box, 3, 1, 1, 4);

    // Create DISPLAY and ENTRY
    // single line text entry widget
    let display = Entry::builder()
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build();
    grid.attach(&display, 0, 0, 3 ,1);

    // --> BUTTONS
    // --> ROW 1
    // Create the first button and put it into the grid at (0, 0, x, x)
    let button_1 = gtk::Button::with_label("Button 1");
    button_1.connect_clicked(move |_| {
        let i: u8 = rand::random();
        println!("Button 1: {}", i)
    });
    grid.attach(&button_1, 0, 1, 1, 1);

    // Create the second button and put it into the grid at (1, 0, x, x)
    let button_2 = gtk::Button::with_label("Button 2");
    button_2.connect_clicked(move |_| println!("Button 2"));
    grid.attach(&button_2, 1, 1, 1, 1);

    let button_3 = gtk::Button::with_label("Button 3");
    button_3.connect_clicked(move |_| println!("Button 3"));
    grid.attach(&button_3, 2, 1, 1, 1);
    
    // --> ROW 2
    let button_4 = gtk::Button::with_label("Button 4");
    button_4.connect_clicked(move |_| println!("Button 4"));
    grid.attach(&button_4, 0, 2, 1, 1);
    
    let button_5 = gtk::Button::with_label("Button 5");
    button_5.connect_clicked(move |_| println!("Button 5"));
    grid.attach(&button_5, 1, 2, 1, 1);
    
    let button_6 = gtk::Button::with_label("Button 6");
    button_6.connect_clicked(move |_| println!("Button 6"));
    grid.attach(&button_6, 2, 2, 1, 1);
    
    // --> ROW 3
    let button_7 = gtk::Button::with_label("Button 7");
    button_7.connect_clicked(move |_| println!("Button 7"));
    grid.attach(&button_7, 0, 3, 1, 1);
    
    let button_8 = gtk::Button::with_label("Button 8");
    button_8.connect_clicked(move |_| println!("Button 8"));
    grid.attach(&button_8, 1, 3, 1, 1);
    
    let button_9 = gtk::Button::with_label("Button 9");
    button_9.connect_clicked(move |_| println!("Button 9"));
    grid.attach(&button_9, 2, 3, 1, 1);
    
    // LABEL
    let counter_label = Label::new(Some("0"));
    grid.attach(&counter_label, 0, 5, 4, 1);

    // --> ROW 4
    let plus_button = gtk::Button::with_label("+");
    plus_button.connect_clicked(glib::clone!(@weak counter_label => move |_| {
        let nb = counter_label.text()
            .parse()
            .unwrap_or(0);
        counter_label.set_text(&format!("{}", nb + 1));
    }));
    grid.attach(&plus_button, 0, 4, 1, 1);
    let button_0 = gtk::Button::with_label("Button 0");
    button_0.connect_clicked(move |_| println!("Button 0"));
    grid.attach(&button_0, 1, 4, 1, 1);
    let minus_button = gtk::Button::with_label("-");
    minus_button.connect_clicked(glib::clone!(@weak counter_label => move |_| {
        let nb = counter_label.text()
            .parse()
            .unwrap_or(0);
        counter_label.set_text(&format!("{}", nb - 1));
    }));
    grid.attach(&minus_button, 2, 4, 1, 1);

    // TIMER
    let time = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    let label_time = gtk::Label::new(None);
    label_time.set_text(&time);
    grid.attach(&label_time, 0, 6, 3, 1);

    // Changing time
    let tick = move || {
        let time = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        label_time.set_text(&time);
        // How it works?
        glib::Continue(true)
    };
    // First digit means rate of time update inseconds
    glib::timeout_add_seconds_local(1, tick);

    // --> ROW 2 COLUMN 4
    // Create the quit button and put it into the grid at (3, 1) with Width = 1 and Height = 4
    let quit_button = gtk::Button::with_label("Quit");
    quit_button.connect_clicked(clone!(@weak window => move |_| 
        unsafe {
            window.destroy()
        }
    ));
    grid.attach(&quit_button, 3, 1, 1, 4);

    window.show_all();
}