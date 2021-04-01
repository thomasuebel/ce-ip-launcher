//
// Module gui
// Thomas Uebel <t.uebel@gmail.com>
//
mod actions;

pub mod gui {
    extern crate gtk;

    use gtk::prelude::*;
    use gtk::{Window};
    use crate::ui::actions::action_handler;

    pub fn setup() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }

        let glade_src = include_str!("../assets/ce-ip-launcher-ui.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let window: gtk::Window = builder.get_object("window.launcher").unwrap();

        let launch_button: gtk::Button = builder.get_object("button.launch").unwrap();

        launch_button.connect_clicked( |button|{
            action_handler::toast();
        });

        window.show_all();
        gtk::main();
    }

}