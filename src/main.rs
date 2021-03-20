use gtk::glib::{clone, MainContext, timeout_future_seconds};
use gtk::prelude::*;
use libretranslate::Language;
use std::env::args;

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
        let builder = gtk::Builder::from_string(include_str!("window.ui"));
        let window: gtk::ApplicationWindow =
            builder.get_object("window").expect("Couldn't get window");
        window.set_application(Some(application));

        let source_text: gtk::TextView = builder
            .get_object("source_text")
            .expect("Couldn't get source_text");
        let target_text: gtk::TextView = builder
            .get_object("target_text")
            .expect("Couldn't get target_text");

        let source_combo_box: gtk::ComboBox = builder
            .get_object("source_combo_box")
            .expect("Couldn't get source_combo_box");
        let target_combo_box: gtk::ComboBox = builder
            .get_object("target_combo_box")
            .expect("Couldn't get target_combo_box");

        let translate_button: gtk::Button = builder
            .get_object("translate_button")
            .expect("Couldn't get translate_button");
        let loading_spinner: gtk::Spinner = builder
            .get_object("loading_spinner")
            .expect("Couldn't get loading_spinner");

        translate_button.connect_clicked(clone!(
            @strong source_text, 
            @strong target_text,
            @strong source_combo_box,
            @strong target_combo_box,
            @strong translate_button,
            @strong loading_spinner 
            => move |_| {
            
            let source_text = source_text.clone();
            let target_text = target_text.clone();

            let source_combo_box = source_combo_box.clone();
            let target_combo_box = target_combo_box.clone();
            let loading_spinner = loading_spinner.clone();

            let main_context = MainContext::default();
            // The main loop executes the asynchronous block
            main_context.spawn_local(async move {
                loading_spinner.start();

                let (start,end) = source_text.get_buffer().get_bounds();
                let input = source_text.get_buffer().get_text(&start, &end, false);
                let source = source_combo_box.get_active_id().unwrap().parse::<Language>().unwrap();
                let target = target_combo_box.get_active_id().unwrap().parse::<Language>().unwrap();

                let output = translate(Some(source), target, input.as_str()).await;

                target_text.get_buffer().set_text(output.as_str());

                loading_spinner.stop();
            });

        }));


        Self { window }
    }
}

async fn translate(source: Option<Language>, target: Language, input: &str) -> String {
    let func = libretranslate::translate(source, target, input).unwrap();

    return func.output;
}