use gtk::AboutDialog;
use gtk::prelude::*;
use gtk::License::Gpl30;

pub fn about() {
	let about_window = AboutDialog::new();
	about_window.set_website_label(Some("gtktranslate"));
	about_window.set_website(Some("https://github.com/skylinecc/gtktranslate"));
	about_window.set_website_label(Some("Gtktranslate Project Page"));
	about_window.set_comments(Some("A GTK3 Google Translate Wrapper"));
	about_window.set_copyright(Some("Copyright © 2020 Skyline Coding Club"));
	about_window.set_license_type(Gpl30);
	about_window.set_wrap_license(false);
	about_window.set_title("About gtktranslate");
	about_window.set_logo_icon_name(Some("gtktranslate.svg"));
    about_window.set_authors(&["Grant Handy"]);
	about_window.add_credit_section(&"Club Members", &[
		"Nicholas Zhang",
		"Ethan Suresh",
		"Alex Ikeda",
		"Evan Ikeda",
		"Corrine Wang",
		"Miguel Oyarzun",
		"Grant Handy",
		"Michael Donnely",
		"Ayush Ranjan",
		"Alex Rose",]);
    about_window.show_all();
}

