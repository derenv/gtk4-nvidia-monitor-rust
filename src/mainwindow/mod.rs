// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (MainWindow)
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
use gtk::{Window, Inhibit};

// Imports
use adwaita::{gio, glib, prelude::*, subclass::prelude::*};
use gio::{Settings, SimpleAction};
use glib::{clone, closure, Object};

// Modules
use crate::{provider::Provider, settingswindow::SettingsWindow, APP_ID};

// GObject wrapper for MainWindow
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
        // Create new window
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
        // Connect closure to re-load stored views (with different settings) when a settings window is closed
        //NOTE: expected return value seems to be broken - look at imp.rs:1772
        self.connect_closure(
            "update-all-views",
            false,
            closure!(move |window: MainWindow| {
                // println!("closure called!"); //TEST

                // Reload views
                // println!("reloading views.."); //TEST
                window.imp().refresh_cards();
                // println!("views reloaded.."); //TEST

                // Return final value
                0
            }),
        );
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
            let gpu_page_layout: Vec<Vec<String>>


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
            self.append_page();
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
            //TODO: check if process exists
            //let defaultAppSystem = Shell.AppSystem.get_default();
            //let nvidiaSettingsApp = defaultAppSystem.lookup_app('nvidia-settings.desktop');
            //let def = shell::Edge::Top;
            //let dd = gio::DesktopAppInfo::from_filename("nvidia-settings.desktop");

            // Check if settings window open
            //NOTE: this never gets set to false again, needs fixing
            match window.imp().get_setting::<bool>("nvidia-settings-open") {
                true => {
                    println!("....Nvidia Settings app already open")//DEBUG
                }
                false => {
                    // Check if provider exists
                    match window.property::<Option<Provider>>("provider") {
                        Some(provider) => {
                            // Open Nvidia Settings
                            match provider.open_settings() {
                                Ok(_) => {
                                    println!("Opening the Nvidia Settings app.."); //DEBUG

                                    // Update state in settings
                                    window.imp().update_setting::<bool>("nvidia-settings-open", true);
                                },
                                Err(err) => println!(
                                    "An error occured: {}",
                                    err
                                ),
                            }
                        },
                        None => {
                            println!("Opening the Nvidia Settings app.."); //DEBUG

                            // Check provider type
                            let provider_type: i32 = window.imp().get_setting::<i32>("provider");

                            // Create new provider
                            window.set_property("provider", Some(window.imp().create_provider(provider_type)));

                            // Open Nvidia Settings
                            match window.property::<Option<Provider>>("provider") {
                                Some(provider) => {
                                    // Open Nvidia Settings
                                    match provider.open_settings() {
                                        Ok(_) => {
                                            // Update state in settings
                                            window.imp().update_setting::<bool>("nvidia-settings-open", true);
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
                }
            }
        }));
        self.add_action(&open_nvidia_settings);

        // Show app settings window
        let open_app_settings: SimpleAction = SimpleAction::new("open_app_settings", None);
        open_app_settings.connect_activate(clone!(@weak self as window => move |_, _| {
            // Check if settings window open
            match window.imp().get_setting::<bool>("app-settings-open") {
                true => {
                    println!("....settings window already open");//DEBUG
                }
                false => {
                    println!("Opening settings window.."); //DEBUG

                    // Update settings
                    window.imp().update_setting::<bool>("app-settings-open", true);

                    // Create an app object
                    let app: adwaita::Application = adwaita::Application::builder().application_id(APP_ID).build();

                    // Create new settings window
                    let new_settings_window: SettingsWindow = SettingsWindow::new(&app, &window);

                    // Show new settings window
                    new_settings_window.show();
                }
            }
        }));
        self.add_action(&open_app_settings);

        // Show About pop-up window
        let about: SimpleAction = SimpleAction::new("about", None);
        about.connect_activate(clone!(@weak self as window => move |_, _| {
            // Check if about window open
            match window.imp().get_setting::<bool>("about-open") {
                true => {
                    println!("....about window already open");//DEBUG
                }
                false => {
                    println!("Opening about window.."); //DEBUG

                    // Update settings
                    window.imp().update_setting::<bool>("about-open", true);

                    // Create window
                    let about_window = Window::builder()
                        .width_request(350)
                        .height_request(250)
                        .build();

                    // Update settings on window close
                    about_window.connect_close_request(
                        move |_| {
                            // println!("Closing about window.."); //DEBUG

                            // Update settings
                            window.imp().update_setting::<bool>("about-open", false);

                            // Tell gtk to continue to default handler
                            Inhibit(false)
                        },
                    );

                    // Show window
                    about_window.show();
                }
            }
        }));
        self.add_action(&about);
    }
}
