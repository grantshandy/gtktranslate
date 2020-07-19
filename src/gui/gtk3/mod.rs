use gtk::prelude::*;
use std::process::Command;
use std::str;
use gtk::AboutDialog;
use gtk::License::Gpl30;

pub fn launch() {
	gtk::init().unwrap_or_else(|_| panic!("GTK could not start!"));


	// Declare windows and export .ui file.
	let builder = gtk::Builder::new_from_string(include_str!("app_window.ui"));
	let main_window: gtk::Window = builder.get_object("app_window").unwrap();
	
	let about_button: gtk::Button = builder.get_object("about-button").unwrap();
	about_button.connect_clicked(move |_| {
		println!("Starting About Dialog");
		
		let p = AboutDialog::new();
        	p.set_website_label(Some("gtktranslate"));
        	p.set_website(Some("https://skylinecc.github.io"));
        	p.set_website_label(Some("Skyline Coding Club Website"));

		p.set_authors(&[
 			"Grant Handy",
        		"Nicholas Zhang",
        		"Aditya Suresh",
        		"Alex Ikeda",
        		"Evan Ikeda",
        	]);
        	
        	p.set_copyright(Some("Copyright © 2020 Skyline Coding Club"));
        	
        	let license = 
"Gtktranslate is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as 
published by the Free Software Foundation; either version 2 of the 
License, or (at your option) any later version.

Gtktranslate is distributed in the hope that it will be useful
but WITHOUT ANY WARRANTY; without even the implied warranty of 
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the 
GNU General Public License for more details.

You should have received a copy of the GNU General Public License 
along with Glade; if not, write to the Free Software 
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, 
MA 02110-1301, USA.";
        	
        	p.set_license_type(Gpl30);
		p.set_license(Some(license));
        	p.set_wrap_license(true);
        	p.set_title("About gtktranslate");
        	p.show_all();
    	});
    	
	let translate_button: gtk::Button = builder.get_object("translate-button").unwrap();
        translate_button.connect_clicked(move |_| {
        
        let mut inputtext: gtk::TextView = builder.get_object("input-text").unwrap();
        let mut inputbuffer: gtk::TextBuffer = inputtext.get_buffer().unwrap();
        let (start,end) = inputbuffer.get_bounds();
        let text = inputbuffer.get_text(&start,&end,false).unwrap();
        let input = text.as_str();
        let mut command = Command::new("/usr/bin/trans");
        let mut cmd = &mut command;
        cmd = cmd.arg("-b");

        let source_language = "en";
        let destination_language = "fr";
        if !source_language.is_empty() && !destination_language.is_empty() {
            cmd = cmd.arg(format!("{}:{}", source_language, destination_language));
        }
        let command = cmd.arg(input);
        let output = cmd.output().unwrap().stdout;
        let output = str::from_utf8(&output).unwrap();
        let mut outputtext: gtk::TextView = builder.get_object("output-text").unwrap();
        let mut outputbuffer: gtk::TextBuffer = outputtext.get_buffer().unwrap();

        outputbuffer.set_text(output);
        
        println!("{} -> {}", input, output);

    });

	main_window.show_all();
	main_window.set_title("Translator");

	//... and to kill the event...

	main_window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
    	});
    
	gtk::main();
}
