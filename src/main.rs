use gtk::prelude::*;
use std::env::args;
use gtk::glib::{clone, MainContext};
use libretranslate::{Translator, Language};

fn main() {
    // Create a new application
    let app = gtk::Application::new(Some("com.DefunctLizard.Libretranslate"), Default::default())
        .expect("Application Initialization failed...");

    app.connect_activate(|app| {
        let window = Window::new(app);
        window.window.present();
    });

    println!("Launching GUI...");
    app.run(&args().collect::<Vec<_>>());
}

struct Window {
    pub window: gtk::ApplicationWindow,
}

impl Window {
    pub fn new(application: &gtk::Application) -> Self {
        let builder = gtk::Builder::from_string(include_str!("textview_editable.ui"));
        let window: gtk::ApplicationWindow =
            builder.get_object("window").expect("Couldn't get window");
        window.set_application(Some(application));

        let source_text: gtk::TextView = builder
            .get_object("source_text")
            .expect("Couldn't get source_text");
        let target_text: gtk::TextView = builder
            .get_object("target_text")
            .expect("Couldn't get target_text");
            
        let translate_button: gtk::Button = builder
            .get_object("translate_button")
            .expect("Couldn't get translate_button");
        let loading_spinner: gtk::Spinner = builder
            .get_object("loading_spinner")
            .expect("Couldn't get loading_spinner");

        translate_button.connect_clicked(clone!(@strong source_text, @strong loading_spinner => move |_| {

        }));

        Self { window }
    }
}
