use gtk::prelude::*;
use std::process::Command;
use std::str;
use gtk::AboutDialog;
use gtk::License::Gpl30;
use glib_sys::g_free;

pub fn launch() {
	gtk::init().unwrap_or_else(|_| panic!("GTK could not start!"));

//	Declare windows and import .ui file.
	let builder = gtk::Builder::new_from_string(include_str!("app_window.ui"));
	let main_window: gtk::Window = builder.get_object("app_window").unwrap();
	let about_window = AboutDialog::new();
	let about_button: gtk::Button = builder.get_object("about-button").unwrap();
	let translate_button: gtk::Button = builder.get_object("translate-button").unwrap();
	let input_lang: gtk::ComboBoxText = builder.get_object("input-lang").unwrap();
	let output_lang: gtk::ComboBoxText = builder.get_object("output-lang").unwrap();

//	execute about_button function.
	about_button.connect_clicked(move |_| {
		println!("Starting About Dialog");

//		Set AboutDialog Preferences.
		about_window.set_website_label(Some("gtktranslate"));
		about_window.set_website(Some("https://skylinecc.github.io"));
		about_window.set_website_label(Some("Skyline Coding Club Website"));
    	about_window.set_comments(Some("A GTK3 Google Translate Wrapper"));
		about_window.set_copyright(Some("Copyright © 2020 Skyline Coding Club"));
		about_window.set_license_type(Gpl30);
		about_window.set_wrap_license(false);
		about_window.set_title("About gtktranslate");
		about_window.set_authors(&[
			"Grant Handy",
			"Nicholas Zhang",
			"Aditya Suresh",
			"Alex Ikeda",
			"Evan Ikeda",
		]);
		about_window.show_all();
	});

//	execute translate_button function.
	translate_button.connect_clicked(move |_| {
        
//		Unwrap input buffer and do some utf-8 coding gymnastics. Seriously, this is stupid, don't do this.
		let mut inputtext: gtk::TextView = builder.get_object("input-text").unwrap();
		let mut inputbuffer: gtk::TextBuffer = inputtext.get_buffer().unwrap();
		let (start,end) = inputbuffer.get_bounds();
		let text = inputbuffer.get_text(&start,&end,false).unwrap();
		let input = text.as_str();
        
//	    	Import "trans" command for translations, and make it brief.
		let mut command = Command::new("gawk -f <(curl -Ls git.io/translate)");
		let mut cmd = &mut command;
	        cmd = cmd.arg("-b");

//		Get Input and Output Languages from ComboBox.
        	let mut active_input = input_lang.get_active_text().unwrap();
        	let mut active_input_str = active_input.as_str();

		let mut active_output = output_lang.get_active_text().unwrap();
		let mut active_output_str = active_output.as_str();	

//		Convert fancy input and output names into simple 2 letter names
		
		let mut source_language = "";
		let mut destination_language"";

		if active_input_str == "English" {
			let mut source_language = "en";
			println!("source language: English");
		};
       	
      		if active_output_str == "French" {
			let mut destination_language = "fr";
			println!("Destination language set: French");
		};
       	
       	if active_output_str == "Spanish" {
			let mut destination_language = "es";
			println!("Destination language set: Spanish");
		};

//	    	If the source and destination languages are entered, set them as parameters to "trans".
		if !source_language.is_empty() && !destination_language.is_empty() {
			cmd = cmd.arg(format!("{}:{}", source_language, destination_language));
		}
        
//	    	Do more utf-8 coding gymnastics just to get our output from "trans" back into the output text box.
		let command = cmd.arg(input);
		let output = cmd.output().unwrap().stdout;
		let output = str::from_utf8(&output).unwrap();
		let mut outputtext: gtk::TextView = builder.get_object("output-text").unwrap();
		let mut outputbuffer: gtk::TextBuffer = outputtext.get_buffer().unwrap();
		outputbuffer.set_text(output);
       
//	    	Print our input -> output into the console.
		println!("{} -> {}", input, output);       	

	});

	main_window.show_all();

	main_window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
	});
    
	gtk::main();
}
