extern crate gtk;

mod gui {
	pub mod gtk3;
}

fn main() {
	println!("Starting GUI...");
	gui::gtk3::launch();
}
