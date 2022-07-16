use glib::clone;
use gtk::prelude::*;
use gtk::Entry;

pub fn build_ui(application: &gtk::Application) {
    // Create a new window, set it's title and default size
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("GTK grid");
    window.set_default_size(200, 120);

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

    // Create DISPLAY and ENTRY
    let display = Entry::builder()
        .margin_start(margin)
        .margin_top(margin)
        .margin_end(margin)
        .margin_bottom(margin)
        .build();
    grid.attach(&display, 0, 0, 3 ,1);
    
    // BUTTONS
    // ROW 1
    // Create the first button and put it into the grid at (0, 0, x, x)
    let button_1 = gtk::Button::with_label("Button 1");
    button_1.connect_clicked(move |_| println!("Button 1"));
    grid.attach(&button_1, 0, 1, 1, 1);

    // Create the second button and put it into the grid at (1, 0, x, x)
    let button_2 = gtk::Button::with_label("Button 2");
    button_2.connect_clicked(move |_| println!("Button 2"));
    grid.attach(&button_2, 1, 1, 1, 1);

    let button_3 = gtk::Button::with_label("Button 3");
    button_3.connect_clicked(move |_| println!("Button 3"));
    grid.attach(&button_3, 2, 1, 1, 1);
    
    // ROW 2
    let button_4 = gtk::Button::with_label("Button 4");
    button_4.connect_clicked(move |_| println!("Button 4"));
    grid.attach(&button_4, 0, 2, 1, 1);
    
    let button_5 = gtk::Button::with_label("Button 5");
    button_5.connect_clicked(move |_| println!("Button 5"));
    grid.attach(&button_5, 1, 2, 1, 1);
    
    let button_6 = gtk::Button::with_label("Button 6");
    button_6.connect_clicked(move |_| println!("Button 6"));
    grid.attach(&button_6, 2, 2, 1, 1);
    
    // ROW 3
    let button_7 = gtk::Button::with_label("Button 7");
    button_7.connect_clicked(move |_| println!("Button 7"));
    grid.attach(&button_7, 0, 3, 1, 1);
    
    let button_8 = gtk::Button::with_label("Button 8");
    button_8.connect_clicked(move |_| println!("Button 8"));
    grid.attach(&button_8, 1, 3, 1, 1);
    
    let button_9 = gtk::Button::with_label("Button 9");
    button_9.connect_clicked(move |_| println!("Button 9"));
    grid.attach(&button_9, 2, 3, 1, 1);

    // ROW 3
    let button_0 = gtk::Button::with_label("Button 0");
    button_0.connect_clicked(move |_| println!("Button 0"));
    grid.attach(&button_0, 1, 4, 1, 1);

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