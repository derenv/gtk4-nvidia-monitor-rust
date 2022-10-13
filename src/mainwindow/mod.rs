// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
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

// Imports
use adwaita::{gio, glib, prelude::*, subclass::prelude::*};
use gio::{Settings, SimpleAction};
use glib::{clone, Object};
use std::ffi::OsStr;

// Modules
use crate::{formatter, processor, property, provider, settingswindow, subprocess, APP_ID};
use formatter::Formatter;
use processor::Processor;
use property::Property;
use provider::Provider;
use settingswindow::SettingsWindow;

// GObject wrapper for Property
glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

/*
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
    /*
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

    /*
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

    /*
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

    /*
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

    /*
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

    /*
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
        /*
        let action_close = SimpleAction::new("close", None);
        action_close.connect_activate(clone!(@weak self as window => move |_, _| {
            window.close();
        }));
        self.add_action(&action_close);
        actions.add_action(&action_close);
        */

        // settings
        //let task1 = self.settings().create_action("task1");

        // Create action from key "open_nvidia_settings" and add to action group "win"
        let open_nvidia_settings: SimpleAction = SimpleAction::new("open_nvidia_settings", None);
        open_nvidia_settings.connect_activate(clone!(@weak self as window => move |_, _| {
            // Get state from settings
            let settings: &Settings = window.settings();
            let app_settings_open: bool = settings.boolean("nvidia-settings-open");

            if !app_settings_open {
                match subprocess::exec_check(&[OsStr::new("nvidia-settings")], None::<&gio::Cancellable>) {
                    Ok(_x) => println!("Opening the Nvidia Settings app.."),
                    Err(y) => println!(
                        "An error occured while opening the Nvidia Settings app: {}",
                        y.message()
                    ),
                };

                // Set state in settings
                settings.set_boolean("nvidia-settings-open", true).expect("Could not set setting.");
            }
        }));
        self.add_action(&open_nvidia_settings);

        // Create action from key "subprocess" and add to action group "win"
        let subprocess: SimpleAction = SimpleAction::new("subprocess", None);
        subprocess.connect_activate(clone!(@weak self as window => move |_, _| {
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
                            println!("Process suceeded, returning: `{}`", stdout_buffer_contents)
                        }
                        Err(err) => panic!("{}", err),
                    },
                    (Some(stdout_buffer), Some(stderr_buffer)) => {
                        match std::str::from_utf8(&stdout_buffer) {
                            Ok(stdout_buffer_contents) => match std::str::from_utf8(&stderr_buffer) {
                                Ok(stderr_buffer_contents) => println!(
                                    "Process suceeded, returning: `{}` but with error: `{}`",
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
        }));
        self.add_action(&subprocess);

        // Create action from key "processor" and add to action group "win"
        let processor: SimpleAction = SimpleAction::new("processor", None);
        processor.connect_activate(clone!(@weak self as window => move |_, _| {
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
        }));
        self.add_action(&processor);

        // Create action from key "formatter_and_property" and add to action group "win"
        let formatter_and_property: SimpleAction =
            SimpleAction::new("formatter_and_property", None);
        formatter_and_property.connect_activate(clone!(@weak self as window => move |_, _| {
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
        }));
        self.add_action(&formatter_and_property);

        // Create action from key "providers" and add to action group "win"
        let providers: SimpleAction = SimpleAction::new("providers", None);
        providers.connect_activate(clone!(@weak self as window => move |_, _| {
            let _gpu_count: i32 = 1;

            // SETTINGS
            let _settings_prov: Provider = Provider::new(|| {
                vec![
                    Property::new(&Processor::new("nvidia-settings", "-q GpuUUID -t"), "utilization.gpu",          "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-settings", "-q GpuUUID -t"), "temperature.gpu",          "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-settings", "-q GpuUUID -t"), "memory.used,memory.total", "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-settings", "-q GpuUUID -t"), "fan.speed",                "", &Formatter::new(), &1),
                ]
            });

            // SMI
            let _smi_prov: Provider = Provider::new(|| {
                vec![
                    Property::new(&Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"), "utilization.gpu",          "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"), "temperature.gpu",          "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"), "memory.used,memory.total", "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"), "fan.speed",                "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"), "power.draw",               "", &Formatter::new(), &1),
                ]
            });

            // SETTINGS & SMI
            let _both_prov: Provider = Provider::new(|| {
                vec![
                    Property::new(&Processor::new("nvidia-settings", "-q GpuUUID -t"),                         "utilization.gpu",          "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-settings", "-q GpuUUID -t"),                         "temperature.gpu",          "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-settings", "-q GpuUUID -t"),                         "memory.used,memory.total", "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-settings", "-q GpuUUID -t"),                         "fan.speed",                "", &Formatter::new(), &1),
                    Property::new(&Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"), "power.draw",               "", &Formatter::new(), &1),
                ]
            });

            // OPTIMUS
            let _optimus_prov: Provider = Provider::new(|| {
                vec![
                    Property::new(&Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"), "utilization.gpu",          "", &Formatter::new(), &1),
                    Property::new(&Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"), "temperature.gpu",          "", &Formatter::new(), &1),
                    Property::new(&Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"), "memory.used,memory.total", "", &Formatter::new(), &1),
                    Property::new(&Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"), "fan.speed",                "", &Formatter::new(), &1),
                    Property::new(&Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"), "power.draw",               "", &Formatter::new(), &1),
                ]
            });
        }));
        self.add_action(&providers);

        // Create action from key "open_app_settings" and add to action group "win"
        let open_app_settings: SimpleAction = SimpleAction::new("open_app_settings", None);
        open_app_settings.connect_activate(clone!(@weak self as window => move |_, _| {
            // Get state from settings
            let settings: &Settings = window.settings();
            let app_settings_open: bool = settings.boolean("app-settings-open");

            if !app_settings_open {
                // Create new application
                let app: adwaita::Application = adwaita::Application::builder().application_id(APP_ID).build();

                // Create settings window
                let settings_window: SettingsWindow = SettingsWindow::new(&app);

                // Show settings window
                settings_window.show();

                // Set state in settings
                settings.set_boolean("app-settings-open", true).expect("Could not set setting.");
            }
            /*
             else {
                // Set settings window as focus
                //
                settings_window.focus();
                self.focus_child();
            }
            */
        }));
        self.add_action(&open_app_settings);

        /*
        // Create action from key "filter" and add to action group "win"
        let action_filter = self.settings().create_action("filter");
        self.add_action(&action_filter);

        // Create action to remove done tasks and add to action group "win"
        let action_remove_done_tasks =
            gio::SimpleAction::new("remove-done-tasks", None);
        action_remove_done_tasks.connect_activate(
            clone!(@weak self as window => move |_, _| {
                let tasks = window.tasks();
                let mut position = 0;
                while let Some(item) = tasks.item(position) {
                    // Get `TaskObject` from `glib::Object`
                    let task_object = item
                        .downcast_ref::<TaskObject>()
                        .expect("The object needs to be of type `TaskObject`.");

                    if task_object.is_completed() {
                        tasks.remove(position);
                    } else {
                        position += 1;
                    }
                }
            }),
        );
        self.add_action(&action_remove_done_tasks);
        */
    }
}
