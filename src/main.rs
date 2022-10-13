// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * main.rs
 *
 * Description:
 * Rust Implementation of Nvidia Gnome Extension
 *
 * Made:
 * 12/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */

// Modules
mod formatter;
mod mainwindow;
mod processor;
mod property;
mod provider;
mod subprocess;
use mainwindow::MainWindow;
mod settingswindow;

// Imports
//use std::env;
//use std::path::Path;
//use libappindicator::{
//    /* SysTray */ AppIndicator, AppIndicatorStatus
//};

use adwaita::prelude::*;
//use gtk::Application;
use adwaita::{
    /* Libraries */ gio,
    /* Application */ Application,
    //    SplitButton
};
use gio::{resources_register_include, Settings};

// Constants
const APP_ID: &str = "com.gtk_d.NvidiaExtensionRust";

// Main Function
fn main() {
    // Resources
    resources_register_include!("nvidiaextensionrust.gresource")
        .expect("Failed to register resources.");

    // Intialise GTK
    gtk::init().expect("Failed to initialise gtk");

    // Create a new application
    let app: Application = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    //app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

/*
//https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/todo/5/main.rs
//https://gtk-rs.org/gtk4-rs/stable/latest/book/todo_3.html
fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}
*/

// Build Function
fn build_ui(app: &Application) {
    // Create a new custom window and show it
    let window: MainWindow = MainWindow::new(app);
    window.show();

    /*
    // Menu Child
    let menu: Menu = Menu::new();
    let item: Menu = Menu::new();
    item.append(Some("Utilisation"), Some("app.util"));
    item.append(Some("Temperature"), Some("app.temp"));
    item.append(Some("Memory Usage"), Some("app.memo"));
    item.append(Some("Fan Speed"), Some("app.fans"));
    menu.append_submenu(Some("Item 1"), &item);
    menu.append(Some("SMI"), Some("app.smi"));
    menu.append(Some("Settings"), Some("app.settings"));
    app.set_menubar(Some(&menu));

    // App Indicator
    //let mut indicator = AppIndicator::new("Nvidia App", "");
    //indicator.set_status(AppIndicatorStatus::Active);
    //let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");
    //indicator.set_icon_theme_path(icon_path.to_str().unwrap());
    //indicator.set_icon_full("rust-logo", "icon");
    //indicator.set_menu(&mut menu);

    // Create Parent window
    let window: ApplicationWindow = ApplicationWindow::new(app);
    window.set_title(Some("Nvidia App"));
    window.set_default_size(400, 400);
    window.set_show_menubar(true);

    // Add children to window
    //window.set_child(Some(&button1));
    //window.set_child(Some(&button2));
    //window.set_child(Some(&button3));
    //window.set_child(Some(&button4));
    //window.set_child(Some(&button5));

    // Present window
    window.show();
    */
}
