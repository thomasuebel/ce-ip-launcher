pub mod action_handler {
    extern crate gtk;

    use gtk::prelude::*;
    use gtk::{MessageDialog, MessageType, Window, ButtonsType, DialogFlags};

    pub fn toast() {
        let dialog = MessageDialog::new(None::<&Window>,
                                        DialogFlags::empty(),
                                        MessageType::Info,
                                        ButtonsType::Ok,
                                        "Launching Codename: Eagle");
        dialog.show();
    }
}