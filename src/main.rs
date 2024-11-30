use std::fs;

use config::{Action, Config};
use gtk4::gio::ActionEntryBuilder;
use gtk4::{self as gtk, gdk, Label};
use gtk4::{glib, Align, Application, ApplicationWindow, Button, Orientation};
use gtk4::{prelude::*, CssProvider};

mod config;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("git.michihupf.poweroffrs")
        .build();

    app.connect_activate(build_ui);

    let config = Config::load();
    app.set_accels_for_action("win.poweroff", &[config.bindp.as_str()]);
    app.set_accels_for_action("win.reboot", &[config.bindr.as_str()]);
    app.set_accels_for_action("win.bootwin", &[config.bindw.as_str()]);

    app.run()
}

macro_rules! gtk_action {
    ($action:expr, $name:literal) => {
        ActionEntryBuilder::new($name)
            .activate(move |_: &ApplicationWindow, _, _| {
                $action.perform();
            })
            .build()
    };
}

fn build_ui(app: &Application) {
    // load the config
    let config = Config::load();

    // create window
    let window = ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .default_width(config.width)
        .default_height(config.height)
        .css_name("body")
        .build();

    let provider = CssProvider::new();
    let css = match std::env::var("HOME")
        .ok()
        .and_then(|home| fs::read(home + "/.config/poweroff-rs/style.css").ok())
    {
        Some(css) => css,
        None => {
            println!("No style.css in config directory. Using default styling.");
            Vec::from(include_bytes!("../config/style.css"))
        }
    };
    provider.load_from_data(String::from_utf8(css).unwrap().as_str());

    // apply the style
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(Align::Center)
        .valign(Align::Center)
        .spacing(24)
        .css_name("container")
        .build();

    let title = Label::builder()
        .label(config.title)
        .halign(Align::Center)
        .css_name("title")
        .build();
    container.append(&title);

    let btn_ctn = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .height_request(100)
        .spacing(24)
        .build();

    let poweroff = Button::builder()
        .label("Power off")
        .icon_name("system-shutdown")
        .width_request(100)
        .css_classes(["pbtn", "p-btn"])
        .build();
    poweroff.connect_clicked(|_| Action::Poweroff.perform());
    btn_ctn.append(&poweroff);

    let reboot = Button::builder()
        .label("Reboot")
        .icon_name("system-reboot")
        .width_request(100)
        .css_classes(["pbtn", "r-btn"])
        .build();
    reboot.connect_clicked(|_| Action::Reboot.perform());
    btn_ctn.append(&reboot);

    let winreboot = Button::builder()
        .label("Reboot to Windows")
        .icon_name("poweroff-rs")
        .width_request(100)
        .css_classes(["pbtn", "wr-btn"])
        .build();
    winreboot.connect_clicked(move |_| Action::BootWindows(config.wbentry).perform());
    btn_ctn.append(&winreboot);

    let action_poweroff = gtk_action!(Action::Poweroff, "poweroff");
    let action_reboot = gtk_action!(Action::Reboot, "reboot");
    let action_winreboot = gtk_action!(Action::BootWindows(config.wbentry), "bootwin");

    window.add_action_entries([action_poweroff, action_reboot, action_winreboot]);

    container.append(&btn_ctn);
    window.set_child(Some(&container));
    window.present();
}
