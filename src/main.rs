use gtk::prelude::*;
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
        let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
        window.set_application(Some(application));

		Self {
			window,
		}
	}
}
