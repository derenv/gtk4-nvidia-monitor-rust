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
mod custom_button;
//use custom_button::CustomButton;
mod processor;
mod property;
mod subprocess;
use property::Property;
mod formatter;
use formatter::Formatter;

// Imports
use gtk::prelude::*;
use gtk::{
    /* Libraries */ gio, /* Application */ Application, ApplicationWindow,
    /* Widgets */ Button,
};
use processor::Processor;
use std::ffi::OsStr;
//use std::env;
//use std::path::Path;
//use libappindicator::{
//    /* SysTray */ AppIndicator, AppIndicatorStatus
//};

// Constants
const APP_ID: &str = "org.gtk_rs.NvidiaExtensionRust";

// Main Function
fn main() {
    gtk::init().expect("Failed to initialise gtk");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// Build Function
fn build_ui(app: &Application) {
    // Button Child 1: exec_check (subprocess) launch nvidia-settings
    let button1 = Button::builder()
        .label("Open Settings")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // Connect to "clicked" signal of `button`
    button1.connect_clicked(move |_| {
        match subprocess::exec_check(&[OsStr::new("nvidia-settings")], None::<&gio::Cancellable>) {
            Ok(_x) => println!("Opening the Nvidia Settings app.."),
            Err(y) => println!(
                "An error occured while opening the Nvidia Settings app: {}",
                y.message()
            ),
        };
    });
    // Button Child 2: exec_communicate (subprocess) ask for GPU data
    let button2 = Button::builder()
        .label("Get GPU Names")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // Connect to "clicked" signal of `button`
    button2.connect_clicked(move |_| {
        match subprocess::exec_communicate(
            &[
                OsStr::new("nvidia-settings"),
                OsStr::new("-q"),
                OsStr::new("GpuUUID"),
                OsStr::new("-t"),
            ],
            None::<&gio::Cancellable>,
        ) {
            Ok(return_val) => match return_val {
                (None, None) => println!("no stdout or stderr, something went really wrong..."),
                (None, Some(stderr_buffer)) => match std::str::from_utf8(&stderr_buffer) {
                    Ok(stderr_buffer_contents) => {
                        println!("Process failed with error: {}", stderr_buffer_contents)
                    }
                    Err(err) => panic!("{}", err),
                },
                (Some(stdout_buffer), None) => match std::str::from_utf8(&stdout_buffer) {
                    Ok(stdout_buffer_contents) => {
                        println!("Process suceeded, returning: {}", stdout_buffer_contents)
                    }
                    Err(err) => panic!("{}", err),
                },
                (Some(stdout_buffer), Some(stderr_buffer)) => {
                    match std::str::from_utf8(&stdout_buffer) {
                        Ok(stdout_buffer_contents) => match std::str::from_utf8(&stderr_buffer) {
                            Ok(stderr_buffer_contents) => println!(
                                "Process suceeded, returning: {} but with error: {}",
                                stdout_buffer_contents, stderr_buffer_contents
                            ),
                            Err(err) => panic!("{}", err),
                        },
                        Err(err) => panic!("{}", err),
                    }
                }
            },
            Err(err) => println!("something went wrong: {}", err),
        };
    });
    // Button Child 3: Processor ask for GPU data
    let button3 = Button::builder()
        .label("Get GPU Names")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // Connect to "clicked" signal of `button`
    button3.connect_clicked(move |_| {
        let p: Processor = Processor::new("nvidia-settings", "-q GpuUUID -t");

        //NOTE: Leaving this here for future use..
        //p.set_property("base-call", "nvidia-settings");
        //p.set_property("call", "nvidia-settings");
        //p.set_property("tail-call", "t");

        match p.process() {
            Ok(output) => match output {
                Some(valid_output) => println!("Process suceeded, returning: `{}`", valid_output),
                None => println!("Process encountered an unknown error.."),
            },
            Err(err) => println!("Process encountered an error, returning: `{}`", err),
        }
    });
    // Button Child 4: formatter test
    let button4 = Button::builder()
        .label("Get GPU Names")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // Connect to "clicked" signal of `button`
    button4.connect_clicked(move |_| {
        // GENERIC
        let proc: Processor = Processor::new("nvidia-settings", "-q GpuUUID -t");
        let form: Formatter = Formatter::new();
        let p: Property = Property::new(&proc, "", "", &form, &1);

        let vecc: Vec<Vec<String>> = vec![
            vec!["1.68".to_string(), "2.01".to_string()],
            vec!["3.83".to_string(), "4.22".to_string()],
        ];
        match p.parse(vecc, |input: Vec<String>| {
            Some(input.get(0).unwrap().to_string())
        }) {
            Some(results) => {
                println!("size: {}", results.len());
                for item in results {
                    println!("item: {}", item);
                }
            }
            None => println!("Something's gone really wrong!"),
        }

        // PERCENT
        let proc: Processor = Processor::new("nvidia-settings", "-q GpuUUID -t");
        let form: Formatter = Formatter::new();
        let p: Property = Property::new(&proc, "", "", &form, &1);
        let vecc: Vec<Vec<String>> = vec![
            vec!["1.68".to_string(), "2.01".to_string()],
            vec!["3.83".to_string(), "4.22".to_string()],
        ];
        match p.parse(vecc, |input: Vec<String>| {
            // Grab input
            let mut output: String = input.get(0).unwrap().to_string();

            // Apply formatting
            output.push('%');

            // Return result
            Some(output)
        }) {
            Some(results) => {
                println!("size: {}", results.len());
                for item in results {
                    println!("item: {}", item);
                }
            }
            None => println!("Something's gone really wrong when formatting GENERIC info"),
        }

        // POWER
        let proc: Processor = Processor::new("nvidia-settings", "-q GpuUUID -t");
        let form: Formatter = Formatter::new();
        let p: Property = Property::new(&proc, "", "", &form, &1);
        let vecc: Vec<Vec<String>> = vec![
            vec!["1.68".to_string(), "2.01".to_string()],
            vec!["3.83".to_string(), "4.22".to_string()],
        ];
        match p.parse(vecc, |input: Vec<String>| {
            // Grab input
            let input_str: String = input.get(0).unwrap().to_string();

            // Convert to float
            match input_str.parse::<f64>() {
                Ok(parsed_value) => {
                    // Round down to nearest integer
                    let rounded_value: f64 = parsed_value.floor();

                    // Convert to string
                    let mut output: String = rounded_value.to_string();

                    // Apply formatting
                    output.push('W');

                    // Return result
                    Some(output)
                }
                Err(_) => {
                    //this should catch "" etc
                    println!("Not a valid number");

                    None
                }
            }
        }) {
            Some(results) => {
                println!("size: {}", results.len());
                for item in results {
                    println!("item: {}", item);
                }
            }
            None => println!("Something's gone really wrong when formatting POWER info"),
        }

        // MEMORY
        let proc: Processor = Processor::new("nvidia-settings", "-q GpuUUID -t");
        let form: Formatter = Formatter::new();
        let p: Property = Property::new(&proc, "", "", &form, &1);
        let vecc: Vec<Vec<String>> = vec![
            vec!["1.68".to_string(), "2.01".to_string()],
            vec!["3.83".to_string(), "4.22".to_string()],
        ];
        match p.parse(vecc, |input: Vec<String>| {
            // Grab input
            let current: String = input.get(0).unwrap().to_string();
            let max: String = input.get(1).unwrap().to_string();

            // Convert to float
            match current.parse::<f64>() {
                Ok(parsed_current) => {
                    match max.parse::<f64>() {
                        Ok(parsed_max) => {
                            // Calculate total memory usage
                            let usage: f64 = (parsed_current / parsed_max) * 100.0;

                            // Round down to nearest integer
                            let rounded_value: f64 = usage.floor();

                            // Convert to string
                            let mut output: String = rounded_value.to_string();

                            // Apply formatting
                            output.push('%');

                            // Return result
                            Some(output)
                        }
                        Err(_) => {
                            //this should catch "" etc
                            println!("Not a valid number");

                            None
                        }
                    }
                }
                Err(_) => {
                    //this should catch "" etc
                    //TODO: fix this!
                    println!("Not a valid number");

                    None
                }
            }
        }) {
            Some(results) => {
                println!("size: {}", results.len());
                for item in results {
                    println!("item: {}", item);
                }
            }
            None => println!("Something's gone really wrong when formatting MEMORY info"),
        }

        // TEMPERATURE
        let proc: Processor = Processor::new("nvidia-settings", "-q GpuUUID -t");
        let form: Formatter = Formatter::new();
        let p: Property = Property::new(&proc, "", "", &form, &1);
        let vecc: Vec<Vec<String>> = vec![
            vec!["1.68".to_string(), "2.01".to_string()],
            vec!["3.83".to_string(), "4.22".to_string()],
        ];
        match p.parse(vecc, |input: Vec<String>| {
            // Grab input
            let mut output: String = input.get(0).unwrap().to_string();

            //TODO: needs moved to settings
            #[derive(Debug, PartialEq, Eq)]
            enum TemperatureUnit {
                CELCIUS = 0,
                FAHRENHEIT = 1,
            }
            let current_unit: TemperatureUnit = TemperatureUnit::CELCIUS;
            //let current_unit: TemperatureUnit = TemperatureUnit::FAHRENHEIT;

            // Apply formatting
            if current_unit == TemperatureUnit::CELCIUS {
                // Apply temperature unit
                output.push(char::from_u32(0x00B0).unwrap());
                output.push('C');
            } else if current_unit == TemperatureUnit::FAHRENHEIT {
                match output.parse::<f64>() {
                    Ok(temp) => {
                        // Convert to fahrenheit
                        let fahrenheit_temp: f64 = temp * 9.0 / 5.0 + 32.0;

                        // Round down to nearest integer
                        let rounded_value: f64 = fahrenheit_temp.floor();

                        // Convert to string
                        let mut f_output: String = rounded_value.to_string();

                        // Apply temperature unit
                        f_output.push(char::from_u32(0x00B0).unwrap());
                        f_output.push('F');

                        // Return result
                        Some(f_output)
                    }
                    Err(_) => {
                        //this should catch "" etc
                        println!("Not a valid number");

                        None
                    }
                };
            }

            // Return result
            Some(output)
        }) {
            Some(results) => {
                println!("size: {}", results.len());
                for item in results {
                    println!("item: {}", item);
                }
            }
            None => println!("Something's gone really wrong when formatting TEMPERATURE info"),
        }
    });

    // Menu Child
    let menu = gio::Menu::new();
    let item = gio::Menu::new();
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
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Nvidia App"));
    window.set_default_size(400, 400);
    window.set_show_menubar(true);

    // Add children to window
    //window.set_child(Some(&button1));
    //window.set_child(Some(&button2));
    //window.set_child(Some(&button3));
    window.set_child(Some(&button4));

    // Present window
    window.show();
}
