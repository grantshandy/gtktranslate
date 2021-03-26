use gtk::glib::{clone, MainContext};
use gtk::prelude::*;
use libretranslate::Language;
use std::env::args;


fn main() {
    // Create a new application
    let app = gtk::Application::new(Some("com.DefunctLizard.Gtktranslate"), Default::default())
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

        let source_scrolled_window: gtk::ScrolledWindow = builder
            .get_object("source_scrolled_window")
            .expect("Couldn't get source_scrolled_window");
        source_scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);

        let target_scrolled_window: gtk::ScrolledWindow = builder
        .get_object("target_scrolled_window")
        .expect("Couldn't get target_scrolled_window");
        target_scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);


        let source_text: gtk::TextView = builder
            .get_object("source_text")
            .expect("Couldn't get source_text");
        source_text.set_wrap_mode(gtk::WrapMode::Word);
        let target_text: gtk::TextView = builder
            .get_object("target_text")
            .expect("Couldn't get target_text");
        target_text.set_wrap_mode(gtk::WrapMode::Word);

        let source_combo_box: gtk::ComboBox = builder
            .get_object("source_combo_box")
            .expect("Couldn't get source_combo_box");
        let target_combo_box: gtk::ComboBox = builder
            .get_object("target_combo_box")
            .expect("Couldn't get target_combo_box");

        source_combo_box.set_active_id(Some("detect"));
        target_combo_box.set_active_id(Some(Language::default().as_code()));

        let translate_button: gtk::Button = builder
            .get_object("translate_button")
            .expect("Couldn't get translate_button");
        let loading_spinner: gtk::Spinner = builder
            .get_object("loading_spinner")
            .expect("Couldn't get loading_spinner");
        let no_connection_dialog: gtk::MessageDialog = builder
            .get_object("no_connection")
            .expect("Couldn't get no_connection dialog");
        
        no_connection_dialog.connect_response(move |d: &gtk::MessageDialog, _: gtk::ResponseType| {
            d.hide();
        });

        translate_button.connect_clicked(clone!(
            @strong source_text, 
            @strong target_text,
            @strong source_combo_box,
            @strong target_combo_box,
            @strong translate_button,
            @strong loading_spinner,
            @strong no_connection_dialog,
            => move |_| {
            
            let source_text = source_text.clone();
            let target_text = target_text.clone();

            let source_combo_box = source_combo_box.clone();
            let target_combo_box = target_combo_box.clone();
            let loading_spinner = loading_spinner.clone();
            let no_connection_dialog = no_connection_dialog.clone();

            let main_context = MainContext::default();

            main_context.spawn_local(async move {
                loading_spinner.start();

                let (start,end) = source_text.get_buffer().get_bounds();
                let input = source_text.get_buffer().get_text(&start, &end, false);

                let source: Option<Language> = match source_combo_box.get_active_id().unwrap().as_str() {
                    "detect" => None,
                    _ => Some(source_combo_box.get_active_id().unwrap().parse::<Language>().unwrap()),
                };

                let target: Language = match target_combo_box.get_active_id() {
                    Some(target) => target.parse::<Language>().unwrap(),
                    None => Language::default(),
                };

                let output = match libretranslate::translate(source, target, input).await {
                    Ok(output) => output.output,
                    Err(_) => {
                        no_connection_dialog.show();
                        String::from("No connection")
                    },
                };

                target_text.get_buffer().set_text(output.as_str());

                loading_spinner.stop();
            });
        }));

        Self { window }
    }
}