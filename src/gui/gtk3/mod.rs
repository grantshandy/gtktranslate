use gtk::prelude::*;
use std::process::Command;
use std::str;
use gtk::AboutDialog;
use gtk::License::Gpl30;
use gtk::Inhibit;

#[allow(unused_mut)]

pub fn launch() {
	gtk::init().unwrap_or_else(|_| panic!("GTK could not start!"));

//	Import app_window.ui and set the app_window as main window.
	let builder = gtk::Builder::new_from_string(include_str!("app_window.ui"));
	let main_window: gtk::Window = builder.get_object("app_window").unwrap();

//	Import translate, about and verbose buttons from builder.
	let translate_button: gtk::Button = builder.get_object("translate-button").unwrap();
	let about_button: gtk::Button = builder.get_object("about-button").unwrap();
	let verbose_button: gtk::ToggleButton = builder.get_object("verbose-button").unwrap();
	
//	Execute about_button function.
	about_button.connect_clicked(move |_| {
		println!("Starting About Dialog");
		let about_window = AboutDialog::new();
		about_window.set_website_label(Some("gtktranslate"));
		about_window.set_website(Some("https://github.com/skylinecc/gtktranslate"));
		about_window.set_website_label(Some("Gtktranslate Project Page"));
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
	
	
	
//	Execute translate_button function.
    translate_button.connect_clicked(move |_| {
        
        let mut inputtext: gtk::TextView = builder.get_object("input-text").unwrap();
        let mut inputbuffer: gtk::TextBuffer = inputtext.get_buffer().unwrap();
        let (start,end) = inputbuffer.get_bounds();
        let text = inputbuffer.get_text(&start,&end,false).unwrap();
        let text_str = text.as_str();
        
        let mut command = Command::new("/usr/bin/trans");
        let mut cmd = &mut command;
        cmd = cmd.arg("--no-ansi");

	    let source_language = "en";
        let destination_language = "fr";
	
	    if !verbose_button.get_active() {
	        cmd = cmd.arg("-b")
	    }

        if !source_language.is_empty() && !destination_language.is_empty() {
            cmd = cmd.arg(format!("{}:{}", source_language, destination_language));
        }
        
        let command = cmd.arg(text_str);
        let output = cmd.output().unwrap().stdout;
        let output = str::from_utf8(&output).unwrap();
        let mut outputtext: gtk::TextView = builder.get_object("output-text").unwrap();
        let mut outputbuffer: gtk::TextBuffer = outputtext.get_buffer().unwrap();
	
        outputbuffer.set_text(output);

        dbg!(output);
    	});

	main_window.show_all();

	//... and to kill the event...

	main_window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
    	});
    
	gtk::main();
}
