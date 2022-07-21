# gtk_grid
![calculator_gtk](https://github.com/antonovmike/gtk_grid/blob/main/Screenshot.png)

Making grid of buttons uing only GTK (no extra files like grid.ui).

Explanation https://dev.to/antonov_mike/making-gtk-keyboard-on-rust-2fma

Using unsafe block in quit_button closure.

grid.attach(&button, 0, 0, 1, 1);

1 - Position from left to the right

2 - Position from top to the bottom

3 - Width

4 - Height

Entry single line text entry widget

Label::new(Some("0")) works and shows some data if linked button has clicked

Timer shows local time

https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Entry.html

----------

How to attach Box to Grid?