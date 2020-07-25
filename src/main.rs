extern crate gtk;
extern crate glib;

mod ui;
mod about;

#[allow(unused_mut)]

fn main() {
	println!("Starting GUI...");
	ui::launch();
}
