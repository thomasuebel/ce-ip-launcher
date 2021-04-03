//
// Module gui
// Thomas Uebel <t.uebel@gmail.com>
//
mod actions;

pub mod gui {
    extern crate gtk;

    use gtk::prelude::*;

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
        let ip_entry: gtk::Entry = builder.get_object("entry.ip").unwrap();

        let ip_entry_clone_for_launch_button = ip_entry.clone();
        launch_button.connect_clicked(move |_| {
            action_handler::launch_codename_eagle(&ip_entry_clone_for_launch_button.get_buffer().get_text());
        });

        let ip_entry_clone_for_entry_keypress = ip_entry.clone();
        ip_entry.connect_key_press_event(move |_,_| {
            action_handler::activate_on_valid_ip(&ip_entry_clone_for_entry_keypress.get_buffer().get_text(), &launch_button)
        });

        window.connect_delete_event( |_,_| {
            action_handler::window_delete()
        });

        window.show_all();
    }

}