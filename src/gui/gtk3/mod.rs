use gtk::prelude::*;
use std::process::Command;
use std::str;

pub fn launch() {
	gtk::init().unwrap_or_else(|_| panic!("GTK could not start!"));

	let builder = gtk::Builder::new_from_string(include_str!("app_window.ui"));
	let window: gtk::Window = builder.get_object("app_window").unwrap();

    let translatebutton: gtk::Button = builder.get_object("translate-button").unwrap();
        translatebutton.connect_clicked(move |_| {
        
        let mut inputtext: gtk::TextView = builder.get_object("input-text").unwrap();
        let mut inputbuffer: gtk::TextBuffer = inputtext.get_buffer().unwrap();
        let (start,end) = inputbuffer.get_bounds();
        let text = inputbuffer.get_text(&start,&end,false).unwrap();
        let text_str = text.as_str();
        let mut command = Command::new("/usr/bin/trans");
        let mut cmd = &mut command;

        cmd = cmd.arg("-b");

        let source_language = "en";
        let destination_language = "fr";
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

	window.show_all();
	window.set_title("Translator");

	//... and to kill the event...

	window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
    	});
    
	gtk::main();
}
