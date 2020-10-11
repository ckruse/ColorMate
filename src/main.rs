extern crate fancy_regex;
#[macro_use]
extern crate lazy_static;

use gettextrs::*;
use gio::prelude::*;
use gtk::prelude::*;

mod config;
mod static_resources;
mod window;
use crate::window::Window;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("colormate", config::LOCALEDIR);
    textdomain("colormate");

    static_resources::init().expect("GResource initialization failed.");

    let app = gtk::Application::new(Some("de.wwwtech.ColorMate"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        let window = Window::new();

        window.widget.set_application(Some(app));
        app.add_window(&window.widget);

        window.init();

        window.widget.present();
    });

    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}
