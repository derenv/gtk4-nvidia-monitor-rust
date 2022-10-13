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
use adwaita::subclass::prelude::*;
use adwaita::prelude::*;
use gio::{ Settings, SimpleAction };
use glib::{ Object, clone };
use adwaita::{ gio, glib };

// Modules
use crate::{APP_ID};

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
