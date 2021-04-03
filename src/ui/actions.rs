pub mod action_handler {
    extern crate glib;
    extern crate gtk;

    use glib::signal::Inhibit;
    use gtk::prelude::*;
    use gtk::{Button, ButtonsType, DialogFlags, MessageDialog, MessageType, Window};
    use std::process::Command;

    pub fn toast(message: &str) {
        let dialog = MessageDialog::new(
            None::<&Window>,
            DialogFlags::MODAL,
            MessageType::Info,
            ButtonsType::Ok,
            message,
        );
        dialog.connect_response(|dialog, _| {
            dialog.close();
        });
        dialog.show_all();
    }

    pub fn window_delete() -> Inhibit {
        gtk::main_quit();
        Inhibit(false)
    }

    pub fn activate_on_valid_ip(host: &str, button: &Button) -> Inhibit {
        let dots = host.matches(".")
                .count();
        button.set_sensitive(dots == 3);
        Inhibit(false)
    }

    pub fn launch_codename_eagle(host: &str) {
        let port: String = ":24711".to_owned();

        if cfg!(target_os = "windows") {
            Command::new("Game.exe")
                .arg("+connect")
                .arg(format!("{}{}", host, port))
                .output()
                .expect("failed to launch Codename Eagle.");
        } else {
            toast("Not implemented for this OS.")
        }
    }
}
