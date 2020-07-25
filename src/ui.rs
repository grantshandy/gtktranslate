use gtk::prelude::*;
use std::str;
use gtk::Inhibit;
use std::process::Command;
use std::collections::HashMap;
use crate::about::*;

pub fn launch() {
	gtk::init().unwrap_or_else(|_| panic!("GTK could not start!"));

//	Import app_window.ui and set the app_window as main window.
	let builder = gtk::Builder::new_from_string(include_str!("app_window.ui"));
	let main_window: gtk::Window = builder.get_object("app_window").unwrap();

//	Import translate, about and verbose buttons from builder.
	let translate_button: gtk::Button = builder.get_object("translate-button").unwrap();
	let verbose_button: gtk::ToggleButton = builder.get_object("verbose-button").unwrap();
	let header_bar: gtk::HeaderBar = builder.get_object("header-bar").unwrap();
	let about_button: gtk::Button = builder.get_object("about-button").unwrap();
	
	let mut input_lang_box: gtk::ComboBoxText = builder.get_object("input-lang").unwrap();
	let mut output_lang_box: gtk::ComboBoxText = builder.get_object("output-lang").unwrap();
	
//	Execute about_button function.
	about_button.connect_clicked(move |_| {
		crate::about::about();
	});
	
//	Set header bar settings.
	header_bar.set_has_subtitle(true);
	header_bar.set_title(Some("Translator"));
	header_bar.set_subtitle(Some("Detect -> English"));
	
	let mut lang_codes = HashMap::new();
	lang_codes.insert("English", "en");
	lang_codes.insert("French", "fr");
	lang_codes.insert("Italian", "it");
	lang_codes.insert("Spanish", "es");
	lang_codes.insert("Detect", "");
	
	if let Some(theme) = gtk::IconTheme::get_default() {
		theme.add_resource_path("/usr/share/icons/hicolor/apps/scalable/gtktranslate.svg");
	}
	
	gtk::Window::set_default_icon_from_file("/usr/share/icons/hicolor/apps/scalable/gtktranslate.svg");

//	Execute translate_button function.
	translate_button.connect_clicked(move |_| {

//		Get input from input-text
		let mut inputtext: gtk::TextView = builder.get_object("input-text").unwrap();
		let mut inputbuffer: gtk::TextBuffer = inputtext.get_buffer().unwrap();
		let (start,end) = inputbuffer.get_bounds();
		let text = inputbuffer.get_text(&start,&end,false).unwrap();
		let input = text.as_str();

//		Import and start the trans command with its arguments.
		let mut cmd = Command::new("/usr/bin/trans");
		let mut cmd = &mut cmd;
		cmd = cmd.arg("--no-ansi");
		
		if !verbose_button.get_active() {
			cmd = cmd.arg("-b")
		}
		
//		Set the input and output languages!
//		Import active input and output text.
		let mut active_input = input_lang_box.get_active_text().unwrap();
		let mut active_output = output_lang_box.get_active_text().unwrap();

//		Convert active inputs and outputs to str.
		let mut active_input_str = active_input.as_str();
		let mut active_output_str = active_output.as_str();

//		Set default languages when starting.
		let mut input_lang_code = "";
		let mut output_lang_code = "en";

//		Convert active inputs to lang_codes.
		match lang_codes.get(active_output_str) {
			Some(code) => output_lang_code = code,
			None => println!("{} has no code.", active_output_str)
		}
		
		match lang_codes.get(active_input_str) {
			Some(code) => input_lang_code = code,
			None => println!("{} has no code.", active_input_str)
		}
		
		if !input_lang_code.is_empty() && !output_lang_code.is_empty() {
			cmd = cmd.arg(format!("{}:{}", input_lang_code, output_lang_code));
		}
		
		let cmd = cmd.arg(input);
		let output = cmd.output().unwrap().stdout;
		let output = str::from_utf8(&output).unwrap();
		let mut outputtext: gtk::TextView = builder.get_object("output-text").unwrap();
		let mut outputbuffer: gtk::TextBuffer = outputtext.get_buffer().unwrap();
        
		outputbuffer.set_text(output);
	        
		println!("({}) {} -> ({}) {}", input_lang_code, input, output_lang_code, output);
		
		let mut subtitle = ("{} => {}", input_lang_code, output_lang_code);
//      header_bar.set_subtitle(Some(subtitle));
	});

	main_window.show_all();

	//... and to kill the event...

	main_window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
	});

	gtk::main();
}
