use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::glib::{clone, MainContext};
use once_cell::sync::Lazy;
use libretranslate::{TranslationBuilder, Language};
use std::sync::RwLock;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

static KEY: Lazy<RwLock<String>> = Lazy::new(|| {
    RwLock::new(String::new())
});

static URL: Lazy<RwLock<String>> = Lazy::new(|| {
    RwLock::new(String::from("https://libretranslate.de/"))
});

fn main() {
    let application = gtk::Application::new(
        Some("com.grantshandy.Gtktranslate"),
        Default::default(),
    );

    application.connect_activate(move |application| {
        let window = GtkTranslateWindow::init(application);
        window.window.present();
    });

    let resources_bytes = include_bytes!("../data/resources.gresource");
    let resource_data = glib::Bytes::from(&resources_bytes[..]);
    let res = gio::Resource::from_data(&resource_data).unwrap();
    gio::resources_register(&res);

    application.run();
}

#[derive(Clone)]
struct GtkTranslateWindow {
    application: gtk::Application,
    pub window: gtk::ApplicationWindow,
    input_text: gtk::TextView,
    output_text: gtk::TextView,
    translate_button: gtk::Button,
    lang_input: gtk::ComboBoxText,
    lang_output: gtk::ComboBoxText,
    loading_spinner: gtk::Spinner,
}

impl GtkTranslateWindow {
    pub fn init(application: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(application);
        window.set_default_size(700, 500);
        window.set_title(Some("gtktranslate"));

        let header_bar = gtk::HeaderBar::new();

        let translate_button = gtk::Button::new();
        translate_button.set_label("Translate");
        header_bar.pack_start(&translate_button);

        let loading_spinner = gtk::Spinner::new();
        header_bar.pack_start(&loading_spinner);

        let menu_model = gio::Menu::new();
        menu_model.append(Some("Preferences"), Some("app.preferences"));
        menu_model.append(Some("About gtktranslate"), Some("app.about"));

        let menu_popover = gtk::PopoverMenuBuilder::new()
            .menu_model(&menu_model)
            .build();

        let menu_button = gtk::MenuButtonBuilder::new()
            .popover(&menu_popover)
            .build();

        header_bar.pack_end(&menu_button);

        window.set_titlebar(Some(&header_bar));

        let grid = gtk::Grid::builder()
            .margin_start(6)
            .margin_end(6)
            .margin_top(6)
            .margin_bottom(6)
            .vexpand(true)
            .hexpand(true)
            .row_spacing(6)
            .column_spacing(6)
            .build();
    
        let lang_input = Self::lang_selector();
        lang_input.prepend(Some("auto"), "Detect");
        lang_input.set_active_id(Some("auto"));
        
        let lang_output = Self::lang_selector();

        grid.attach(&lang_input, 0, 0, 1, 1);
        grid.attach(&lang_output, 0, 1, 1, 1);

        let input_text = gtk::TextViewBuilder::new()
            .wrap_mode(gtk::WrapMode::WordChar)
            .top_margin(6)
            .bottom_margin(6)
            .right_margin(6)
            .left_margin(6)
            .editable(true)
            .build();

        let input_scrolled_window = gtk::ScrolledWindowBuilder::new()
            .child(&input_text)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .hexpand(true)
            .vexpand(true)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(6)
            .margin_end(6)
            .build();

        grid.attach(&input_scrolled_window, 1, 0, 1, 1);

        let output_text = gtk::TextViewBuilder::new()
            .wrap_mode(gtk::WrapMode::WordChar)
            .top_margin(6)
            .bottom_margin(6)
            .right_margin(6)
            .left_margin(6)
            .editable(false)
            .cursor_visible(false)
            .build();

        let output_scrolled_window = gtk::ScrolledWindowBuilder::new()
            .child(&output_text)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .hexpand(true)
            .vexpand(true)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(6)
            .margin_end(6)
            .build();

        grid.attach(&output_scrolled_window, 1, 1, 1, 1);

        window.set_child(Some(&grid));

        let mut myself = Self {
            application: application.clone(),
            window,
            input_text,
            output_text,
            translate_button,
            lang_input,
            lang_output,
            loading_spinner,
        };

        myself.actions();
        myself.read_config();

        myself
    }

    fn lang_selector() -> gtk::ComboBoxText {
        let selector = gtk::ComboBoxTextBuilder::new()
            .valign(gtk::Align::Center)
            .vexpand(true)
            .build();
        
        selector.append(Some("en"), "English");
        selector.append(Some("ar"), "Arabic");
        selector.append(Some("zh"), "Chinese");
        selector.append(Some("fr"), "French");
        selector.append(Some("de"), "Italian");
        selector.append(Some("ja"), "Japanese");
        selector.append(Some("pt"), "Portuguese");
        selector.append(Some("ru"), "Russian");
        selector.append(Some("es"), "Spanish");

        selector.set_active_id(Some("en"));

        selector
    }

    fn actions(&mut self) {
        let about = gio::SimpleAction::new("about", None);
        let myself = self.clone();
        about.connect_activate(move |_, _| {
            let d = myself.about_dialog();
            d.present();
        });

        self.application.add_action(&about);

        let translate = gio::SimpleAction::new("translate", None);
        let myself = self.clone();
        translate.connect_activate(move |_, _| {
            myself.translate();
        });

        self.application.add_action(&translate);

        self.translate_button.connect_clicked(move |_| {
            translate.activate(None);
        });

        let preferences = gio::SimpleAction::new("preferences", None);
        let myself = self.clone();
        preferences.connect_activate(move |_, _| {
            myself.settings_dialog();
        });

        self.application.add_action(&preferences);
    }

    fn about_dialog(&self) -> gtk::AboutDialog {
        
        
        gtk::AboutDialogBuilder::new()
            .name("gtktranslate")
            .version("0.4.0")
            .website_label("Website")
            .website("https://github.com/grantshandy/gtktranslate/")
            .comments("A GTK4 Libretranslate GUI")
            .license_type(gtk::License::Gpl30)
            .copyright("Copyright © 2020-2021 Grant Handy")
            .authors(vec!["Grant Handy".to_string()])
            .transient_for(&self.window)
            .application(&self.application)
            .modal(true)
            .logo_icon_name("gtktranslate")
            .build()
    }

    fn error_dialog<T: AsRef<str>>(&self, main: T, secondary: T) {
        let dialog = gtk::MessageDialogBuilder::new()
            .buttons(gtk::ButtonsType::Ok)
            .text(main.as_ref())
            .secondary_text(secondary.as_ref())
            .application(&self.application)
            .transient_for(&self.window)
            .modal(true)
            .build();
        
        dialog.connect_response(
            move |d: &gtk::MessageDialog, _: gtk::ResponseType| {
                d.hide();
            },
        );

        dialog.show();
    }

    fn settings_dialog(&self) {
        let d = gtk::WindowBuilder::new()
            .transient_for(&self.window)
            .modal(true)
            .default_height(130)
            .default_width(300)
            .resizable(false)
            .title("Preferences")
            .build();
        
        let grid = gtk::Grid::builder()
            .margin_start(6)
            .margin_end(6)
            .margin_top(6)
            .margin_bottom(6)
            // .halign(gtk::Align::Center)
            // .valign(gtk::Align::Center)
            .hexpand(true)
            .vexpand(true)
            .row_spacing(6)
            .column_spacing(6)
            .build();
        
        let url_label = gtk::Label::new(Some("url:"));
        url_label.set_vexpand(true);
        let key_label = gtk::Label::new(Some("key:"));
        url_label.set_vexpand(true);

        let url_entry = gtk::Entry::new();
        url_entry.set_hexpand(true);
        let key_entry = gtk::Entry::new();
        key_entry.set_hexpand(true);

        url_entry.set_text(URL.read().unwrap().as_str());
        key_entry.set_text(KEY.read().unwrap().as_str());

        let save_button = gtk::Button::with_label("Save");
        
        let myself = self.clone();
        save_button.connect_clicked(clone!(@weak d, @weak url_entry, @weak key_entry => move |_| {
            let k = key_entry.text().to_string();
            let u = url_entry.text().to_string();

            KEY.write().unwrap().clear();
            KEY.write().unwrap().push_str(&k);

            URL.write().unwrap().clear();
            URL.write().unwrap().push_str(&u);

            let _ = myself.write_config();

            d.close();
        }));

        grid.attach(&url_label, 0, 0, 1, 1);
        grid.attach(&key_label, 0, 1, 1, 1);
        grid.attach(&url_entry, 1, 0, 1, 1);
        grid.attach(&key_entry, 1, 1, 1, 1);
        grid.attach(&save_button, 1, 2, 1, 1);

        d.set_child(Some(&grid));

        d.present();
    }

    fn translate(&self) {
        let main_context = MainContext::default();

        let myself = self.clone();

        main_context.spawn_local(async move {
            myself.loading_spinner.start();

            let source = myself.lang_input.active_id().unwrap().to_string().parse::<Language>().unwrap();
            let target = myself.lang_output.active_id().unwrap().to_string().parse::<Language>().unwrap();

            let (start,end) = myself.input_text.buffer().bounds();
            let input = myself.input_text.buffer().text(&start, &end, false).to_string();

            let output: String = match input.as_str() {
                "" => String::from(""),
                _ => match TranslationBuilder::new()
                    .key(KEY.read().unwrap().as_str())
                    .url(URL.read().unwrap().as_str())
                    .text(input)
                    .from_lang(source)
                    .to_lang(target)
                    .translate()
                    .await {
                    Ok(data) => data.output,
                    Err(error) => {
                        myself.error_dialog("Error Translating", &error.to_string());
                        String::new()
                    }
                }
            };

            myself.output_text.buffer().set_text(output.as_str());

            myself.loading_spinner.stop();
        });
    }

    fn read_config(&self) {
        let mut config_dir = match dirs::config_dir() {
            Some(data) => data,
            None => panic!("Couldn't find config dir"),
        };
    
        config_dir.push("gtktranslate.yaml");
    
        println!("config path: {}", config_dir.to_str().unwrap());
    
        let mut file = match File::open(config_dir.clone()) {
            Ok(data) => data,
            Err(_) => self.write_config(),
        };

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let deserialized: HashMap<String, String> = serde_yaml::from_str(contents.as_str()).unwrap();

        let k = match deserialized.get(&"key".to_string()) {
            Some(data) => data,
            None => {
                self.error_dialog("Couldn't get key", "There was an error with the config");
                std::process::exit(1);
            }
        };

        KEY.write().unwrap().clear();
        KEY.write().unwrap().push_str(k);

        let u = match deserialized.get(&"url".to_string()) {
            Some(data) => data,
            None => {
                self.error_dialog("Couldn't get url", "There was an error with the config");
                std::process::exit(1);
            }
        };

        URL.write().unwrap().clear();
        URL.write().unwrap().push_str(u);
    }

    fn write_config(&self) -> File {
        let mut config_dir = match dirs::config_dir() {
            Some(data) => data,
            None => panic!("Couldn't find config dir"),
        };

        config_dir.push("gtktranslate.yaml");

        let mut file = match File::create(config_dir) {
            Ok(data) => data,
            Err(error) => {
                self.error_dialog("Couldn't create config directory", &error.to_string());
                std::process::exit(1);
            }
        };

        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("key"), KEY.read().unwrap().as_str().to_string());
        map.insert(String::from("url"), URL.read().unwrap().as_str().to_string());

        let s = serde_yaml::to_string(&map).unwrap();

        match file.write_all(s.as_bytes()) {
            Ok(()) => (),
            Err(error) => {
                self.error_dialog("Couldn't Write to Config", &error.to_string());
                std::process::exit(1);
            }
        };

        file
    }
}