// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (SettingsWindow)
 *
 * Made:
 * 10/10/2022
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
use adwaita::{gio, glib, prelude::*, subclass::prelude::*, ComboRow};
use gio::{Settings};
use glib::{clone, Object};
use gtk::{Adjustment, CheckButton, StringList};

// Modules
use crate::APP_ID;

// GObject wrapper for Property
glib::wrapper! {
    pub struct SettingsWindow(ObjectSubclass<imp::SettingsWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

/*
 * Trait Name:
 * SettingsWindow
 *
 * Description:
 * Trait shared by all settings windows
 *
 * Made:
 * 10/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl SettingsWindow {
    /*
     * Name:
     * new
     *
     * Description:
     * Create a new SettingsWindow object
     *
     * Made:
     * 10/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn new(app: &adwaita::Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("`SettingsWindow` should be  instantiable.")
    }

    /*
     * Name:
     * setup_settings
     *
     * Description:
     * Load settings for APP_ID
     *
     * Made:
     * 10/10/2022
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
     * 10/10/2022
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
     * setup_widgets
     *
     * Description:
     * Set up all buttons/drop downs/other widgets
     *
     * Made:
     * 22/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn setup_widgets(&self) {


        // Create adjustment settings for refresh rate SpinButton
        let current_refresh_rate: f64 = self.settings().int("refreshrate").into();
        let adjustment: Adjustment = Adjustment::new(current_refresh_rate, 1.0, 20.0, 1.0, 5.0, 0.0);
        self.imp()
            .refreshrate_input
            .configure(Some(&adjustment), 1.0, 0);

        // Group together Temp-Unit CheckButtons
        let group: &CheckButton = &self.imp().temp_unit_f;
        self.imp()
            .temp_unit_c
            .set_group(Some(group));

        // Retrieve temperature unit from settings
        match self.settings().int("tempformat") {
            0 => {
                self.imp().temp_unit_c.set_active(true);
                self.imp().temp_unit_f.set_active(false);
            },
            1 => {
                self.imp().temp_unit_f.set_active(true);
                self.imp().temp_unit_c.set_active(false);
            },
            _ => panic!("..Unknown temp unit in settings"),
        }

        // Set options for provider
        let items: [&str; 4]  = [
            "Nvidia Settings and Nvidia SMI",
            "Nvidia Settings",
            "Nvidia SMI",
            "Nvidia Optimus",
        ];
        let model = StringList::new(&items);
        self.imp().provider_input.set_model(Some(&model));

        // Set current selected option from settings
        match self.settings().int("provider") {
            0 => {
                self.imp().provider_input.set_selected(0);
            },
            1 => {
                self.imp().provider_input.set_selected(1);
            },
            2 => {
                self.imp().provider_input.set_selected(2);
            },
            3 => {
                self.imp().provider_input.set_selected(3);
            },
            _ => panic!("..Unknown provider value in settings"),
        }
    }

    /*
     * Name:
     * restore_data
     *
     * Description:
     * Restore properties from settings
     *
     * Made:
     * 10/10/2022
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
     * 10/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * TODO
     */
    fn setup_callbacks(&self) {
        // Setup callback for changing provider choice
        self.imp()
            .provider_input
            .connect_selected_notify(clone!(@weak self as window => move |_| {
                // Get new provider choice
                let row: &ComboRow = &window.imp().provider_input;
                let item: u32 = row.selected();

                // Validate and set new provider choice
                match item {
                    0 => {
                        // Validation
                        //TODO: check if provider program exists, call subprocess?

                        // Store chosen provider type
                        let settings = window.imp().settings.get().expect("..Cannot retrieve settings");
                        settings.set_int("provider", 0).expect("..Cannot set `provider` setting");
                    },
                    1 => {
                        // Validation
                        //TODO: check if provider program exists, call subprocess?

                        // Store chosen provider type
                        let settings = window.imp().settings.get().expect("..Cannot retrieve settings");
                        settings.set_int("provider", 1).expect("..Cannot set `provider` setting");
                    },
                    2 => {
                        // Validation
                        //TODO: check if provider program exists, call subprocess?

                        // Store chosen provider type
                        let settings = window.imp().settings.get().expect("..Cannot retrieve settings");
                        settings.set_int("provider", 2).expect("..Cannot set `provider` setting");
                    },
                    3 => {
                        // Validation
                        //TODO: check if provider program exists, call subprocess?

                        // Store chosen provider type
                        let settings = window.imp().settings.get().expect("..Cannot retrieve settings");
                        settings.set_int("provider", 3).expect("..Cannot set `provider` setting");
                    },
                    _ => panic!("..Unknown provider chosen"),
                }
            }));
    }

    /*
     * Name:
     * setup_actions
     *
     * Description:
     * Define all actions from UI
     *
     * Made:
     * 10/10/2022
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
        //
    }
}
