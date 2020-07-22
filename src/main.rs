extern crate gtk;
extern crate glib;

mod gui {
	pub mod gtk3;
}

fn main() {
	println!("Starting GUI...");
	gui::gtk3::launch();
}
