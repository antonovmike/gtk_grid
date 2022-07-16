# gtk_grid
Making grid of buttons uing only GTK (no extra files like grid.ui).

Using unsafe block in quit_button closure.

grid.attach(&button, 0, 0, 1, 1);

1 - Position from left to the right

2 - Position from top to the bottom

3 - Width

4 - Height

Entry single line text entry widget

Label::new(Some("0")) works and shows some data if linked button has clicked

Timer can show start time

https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Entry.html

----------

How to attach Box to Grid?

Make timer show time each second