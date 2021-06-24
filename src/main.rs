use gtk::gio;
use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.search_bar"),
        Default::default(),
    );
    application.connect_activate(move |application| {
        let window = GtkTranslateWindow::init(application);
        window.window.present();
    });
    application.run();
}

#[derive(Clone)]
struct GtkTranslateWindow {
    application: gtk::Application,
    pub window: gtk::ApplicationWindow,
    key: &'static str,
}

impl GtkTranslateWindow {
    pub fn init(application: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(application);
        window.set_default_size(700, 500);
        window.set_title(Some("gtktranslate"));
        window.set_titlebar(Some(&Self::header_bar()));

        let key = "";

        let myself = Self {
            application: application.clone(),
            window,
            key,
        };

        myself.actions();
        myself.build_content();

        return myself;
    }

    fn build_content(&self) {
        let grid = gtk::Grid::builder()
            .margin_start(6)
            .margin_end(6)
            .margin_top(6)
            .margin_bottom(6)
            // .halign(gtk::Align::Center)
            // .valign(gtk::Align::Center)
            .vexpand(true)
            .hexpand(true)
            .row_spacing(6)
            .column_spacing(6)
            .build();
        
        let lang_input = Self::lang_selector();
        let lang_output = Self::lang_selector();

        grid.attach(&lang_input, 0, 0, 1, 1);
        grid.attach(&lang_output, 0, 1, 1, 1);

        self.window.set_child(Some(&grid));
    }

    fn lang_selector() -> gtk::ComboBoxText {
        let selector = gtk::ComboBoxTextBuilder::new()
            .valign(gtk::Align::Center)
            .vexpand(true)
            .build();

        return selector;
    }

    fn actions(&self) {
        let about = gio::SimpleAction::new("about", None);

        let myself = self.clone();
        about.connect_activate(move |_, _| {
            let d = myself.about_dialog();
            d.present();
        });

        self.application.add_action(&about);
    }

    fn header_bar() -> gtk::HeaderBar {
        let header_bar = gtk::HeaderBar::new();
    
        let menu_model = gio::Menu::new();
        menu_model.append(Some("About gtktranslate"), Some("app.about"));
    
        let menu_popover = gtk::PopoverMenuBuilder::new()
            .menu_model(&menu_model)
            .build();
    
        let menu_button = gtk::MenuButtonBuilder::new()
            .popover(&menu_popover)
            .build();

        header_bar.pack_end(&menu_button);
    
        return header_bar;
    }

    fn about_dialog(&self) -> gtk::AboutDialog {
        let about = gtk::AboutDialogBuilder::new()
            .name("gtktranslate")
            .version("0.4.0")
            .website_label("Website")
            .website("https://github.com/grantshandy/gtktranslate/")
            .comments("A GTK4 Libretranslate UI")
            .license_type(gtk::License::Gpl30)
            .copyright("Copyright © 2020-2021 Grant Handy")
            .authors(vec!["Grant Handy".to_string()])
            .transient_for(&self.window)
            .application(&self.application)
            .modal(true)
            .build();
        
        return about;
    }
}
