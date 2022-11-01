// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (Window)
 *
 * Made:
 * 09/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */

// Custom GObjects
mod imp;
use imp::SettingsWindowContainer;

// Imports
use adwaita::{gio, glib, prelude::*, subclass::prelude::*};
use gio::{Settings, SimpleAction};
use glib::{clone, Object};
use std::cell::RefMut;

// Modules
use crate::{APP_ID, provider::Provider, settingswindow::SettingsWindow};

// GObject wrapper for Property
glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

/**
 * Trait Name:
 * MainWindow
 *
 * Description:
 * Trait shared by all main windows
 *
 * Made:
 * 09/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl MainWindow {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new MainWindow object
     *
     * Made:
     * 09/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn new(app: &adwaita::Application) -> Self {
        Object::new(&[("application", app)]).expect("`MainWindow` should be  instantiable.")
    }

    /**
     * Name:
     * setup_settings
     *
     * Description:
     * Load settings for APP_ID
     *
     * Made:
     * 09/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    /**
     * Name:
     * settings
     *
     * Description:
     * Get settings for APP_ID
     *
     * Made:
     * 09/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    /**
     * Name:
     * setup_widgets
     *
     * Description:
     * Set up all widgets
     *
     * Made:
     * 23/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn setup_widgets(&self) {
        //
    }

    /**
     * Name:
     * restore_data
     *
     * Description:
     * Restore properties from settings
     *
     * Made:
     * 09/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * TODO
     */
    fn restore_data(&self) {
        /*
        if let Ok(file) = File::open(data_path()) {
            // Deserialize data from file to vector
            let backup_data: Vec<TaskData> = serde_json::from_reader(file).expect(
                "It should be possible to read `backup_data` from the json file.",
            );

            // Convert `Vec<TaskData>` to `Vec<TaskObject>`
            let task_objects: Vec<TaskObject> = backup_data
                .into_iter()
                .map(TaskObject::from_task_data)
                .collect();

            // Insert restored objects into model
            self.tasks().extend_from_slice(&task_objects);
        }
        */
    }

    /**
     * Name:
     * setup_callbacks
     *
     * Description:
     * Child widget callbacks
     *
     * Made:
     * 09/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * TODO
     */
    fn setup_callbacks(&self) {
        // Setup callback for activation of the entry
        /*
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.new_task();
            }));

        // Setup callback for clicking (and the releasing) the icon of the entry
        self.imp().entry.connect_icon_release(
            clone!(@weak self as window => move |_,_| {
                window.new_task();
            }),
        );
        */
    }

    /**
     * Name:
     * setup_actions
     *
     * Description:
     * Define all actions from UI
     *
     * Made:
     * 09/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * Before we move on to other aspects of actions, let's appreciate a few things
     * that are curious here. The "win" part of "win.close" is the group of the action.
     * But how does GTK know that "win" is the action group of our window? The answer
     * is that it is so common to add actions to windows and applications that there
     * are already two predefined groups available:
     * - "app" for actions global to the application, and
     * - "win" for actions tied to an application window.
     * Also, if we had multiple instances of the same windows we would expect that only
     * the currently focused window will be closed when activating "win.close". And indeed,
     * the "win.close" will be dispatched to the currently focused window. However, that
     * also means that we actually define one action per window instance. If we want to
     * have a single globally accessible action instead, we call add_action on our application instead.
     */
    fn setup_actions(&self) {
        // Create action from key "open_nvidia_settings" and add to action group "win"
        let open_nvidia_settings: SimpleAction = SimpleAction::new("open_nvidia_settings", None);
        open_nvidia_settings.connect_activate(clone!(@weak self as window => move |_, _| {
            // Get state from settings
            let app_settings_open: bool = window.imp().get_setting::<bool>("nvidia-settings-open");

            //let defaultAppSystem = Shell.AppSystem.get_default();
            //let nvidiaSettingsApp = defaultAppSystem.lookup_app('nvidia-settings.desktop');
            //let def = shell::Edge::Top;
            //let dd = gio::DesktopAppInfo::from_filename("nvidia-settings.desktop");

            if !app_settings_open {
                // Grab current stored provider
                let mut provider: Option<Provider> = window.property("provider");

                // Check if provider exists
                match provider {
                    Some(prov) => {
                        // Open Nvidia Settings
                        match prov.open_settings() {
                            Ok(_result) => {
                                println!("Opening the Nvidia Settings app..");
                            },
                            Err(err) => println!(
                                "An error occured: {}",
                                err
                            ),
                        }
                    },
                    None => {
                        // Check provider type
                        let provider_type: i32 = window.imp().get_setting::<i32>("provider");

                        // Create new provider
                        println!("Creating new Provider..");//TEST
                        window.set_property("provider", Some(imp::MainWindow::create_provider(provider_type)));

                        // Grab new provider
                        provider = window.property("provider");

                        // Open Nvidia Settings
                        println!("Opening the Nvidia Settings app..");//TEST
                        match provider {
                            Some(prov) => {
                                // Open Nvidia Settings
                                match prov.open_settings() {
                                    Ok(_result) => {
                                        println!("Opening the Nvidia Settings app..");
                                    },
                                    Err(err) => println!(
                                        "An error occured: {}",
                                        err
                                    ),
                                }
                            },
                            None => panic!("Cannot find `Provider`!")
                        }
                    }
                }

                // Set state in settings
                window.imp().update_setting::<bool>("nvidia-settings-open", true);
            }
        }));
        self.add_action(&open_nvidia_settings);

        /*
        // Create action from key "formatter_and_property" and add to action group "win"
        let formatter_and_property: SimpleAction =
            SimpleAction::new("formatter_and_property", None);
        formatter_and_property.connect_activate(clone!(@weak self as window => move |_, _| {
            // GENERIC
            let proc: Processor = Processor::new("nvidia-settings", "-q GpuUUID -t");
            let form: Formatter = Formatter::new();
            let p: Property = Property::new(&proc, "", "", &form, &1);

            let vecc: Vec<Vec<String>> = vec![
                vec![String::from("1.68"), String::from("2.01")],
                vec![String::from("3.83"), String::from("4.22")],
            ];
            match p.parse(vecc, |input: Vec<String>| {
                Some(String::from(input.get(0).unwrap()))
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
                vec![String::from("1.68"), String::from("2.01")],
                vec![String::from("3.83"), String::from("4.22")],
            ];
            match p.parse(vecc, |input: Vec<String>| {
                // Grab input
                let mut output: String = String::from(input.get(0).unwrap());

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
                vec![String::from("1.68"), String::from("2.01")],
                vec![String::from("3.83"), String::from("4.22")],
            ];
            match p.parse(vecc, |input: Vec<String>| {
                // Grab input
                let input_str: String = String::from(input.get(0).unwrap());

                // Convert to float
                match input_str.parse::<f64>() {
                    Ok(parsed_value) => {
                        // Round down to nearest integer
                        let rounded_value: f64 = parsed_value.floor();

                        // Convert to string
                        let mut output: String = String::from(rounded_value);

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
                vec![String::from("1.68"), String::from("2.01")],
                vec![String::from("3.83"), String::from("4.22")],
            ];
            match p.parse(vecc, |input: Vec<String>| {
                // Grab input
                let current: String = String::from(input.get(0).unwrap());
                let max: String = String::from(input.get(1).unwrap());

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
                                let mut output: String = String::from(rounded_value);

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
                vec![String::from("1.68"), String::from("2.01")],
                vec![String::from("3.83"), String::from("4.22")],
            ];
            match p.parse(vecc, |input: Vec<String>| {
                // Grab input
                let mut output: String = String::from(input.get(0).unwrap());

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
                            let mut f_output: String = String::from(rounded_value);

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
        }));
        self.add_action(&formatter_and_property);
        */

        let open_app_settings: SimpleAction = SimpleAction::new("open_app_settings", None);
        open_app_settings.connect_activate(clone!(@weak self as window => move |_, _| {
            // Borrow (mutable) the window's container
            let mut settings_window_container: RefMut<SettingsWindowContainer> = window.imp().settings_window.borrow_mut();

            // Get state from settings
            settings_window_container.open = window.imp().get_setting::<bool>("app-settings-open");

            // Check if an object is stored
            match &settings_window_container.window {
                Some(_window) => {
                    println!("..window exists");//DEBUG

                    // Check if the window is already open
                    match settings_window_container.open {
                        false => {
                            println!("....opening window");//DEBUG

                            // Create an app object
                            let app: adwaita::Application = adwaita::Application::builder().application_id(APP_ID).build();

                            // Create new settings window
                            let new_settings_window: SettingsWindow = SettingsWindow::new(&app);

                            // Show new settings window
                            new_settings_window.show();

                            // Store object and state back in container
                            settings_window_container.open = true;
                            settings_window_container.window = Some(new_settings_window);
                        },
                        true => {
                            println!("....window already open");//DEBUG
                        },
                    }
                },
                None => {
                    println!("..window does not exist");//DEBUG
                    println!("....opening window");//DEBUG

                    // Create an app object
                    let app: adwaita::Application = adwaita::Application::builder().application_id(APP_ID).build();

                    // Create settings window
                    let new_settings_window: SettingsWindow = SettingsWindow::new(&app);

                    // Show new settings window
                    new_settings_window.show();

                    // Store object and state back in container
                    settings_window_container.open = true;
                    settings_window_container.window = Some(new_settings_window);
                },
            }

            // Set new state in settings
            window.imp().update_setting::<bool>("app-settings-open", settings_window_container.open);
        }));
        self.add_action(&open_app_settings);

        let about: SimpleAction = SimpleAction::new("about", None);
        about.connect_activate(clone!(@weak self as window => move |_, _| {
            // Show About info pop-up
            println!("About pop-up not yet implemented..");//TODO
        }));
        self.add_action(&about);
    }
}
