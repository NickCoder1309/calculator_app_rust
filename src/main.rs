use glib::clone;
// glib and other dependencies are re-exported by the gtk crate
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, glib};

fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);
    // … with a button in it …
    let button = Button::with_label("Hello World!");
    window.set_child(Some(&button));
    window.present();
}

fn main() {
    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
}
