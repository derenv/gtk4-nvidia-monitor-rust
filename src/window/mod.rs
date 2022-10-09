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
use std::ffi::OsStr;
use adwaita::subclass::prelude::*;
use adwaita::{prelude::*};
use gio::Settings;
use glib::{clone, Object};
use adwaita::{gio, glib};

//use crate::utils::data_path;
use crate::{APP_ID, subprocess};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adwaita::Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("`Window` should be  instantiable.")
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

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

    fn task1(&self) {
        match subprocess::exec_check(&[OsStr::new("nvidia-settings")], None::<&gio::Cancellable>) {
            Ok(_x) => println!("Opening the Nvidia Settings app.."),
            Err(y) => println!(
                "An error occured while opening the Nvidia Settings app: {}",
                y.message()
            ),
        };
    }

    fn task2(&self) {
        match subprocess::exec_check(&[OsStr::new("nvidia-settings")], None::<&gio::Cancellable>) {
            Ok(_x) => println!("Opening the Nvidia Settings app.."),
            Err(y) => println!(
                "An error occured while opening the Nvidia Settings app: {}",
                y.message()
            ),
        };
    }

    fn task3(&self) {
        match subprocess::exec_check(&[OsStr::new("nvidia-settings")], None::<&gio::Cancellable>) {
            Ok(_x) => println!("Opening the Nvidia Settings app.."),
            Err(y) => println!(
                "An error occured while opening the Nvidia Settings app: {}",
                y.message()
            ),
        };
    }

    fn task4(&self) {
        match subprocess::exec_check(&[OsStr::new("nvidia-settings")], None::<&gio::Cancellable>) {
            Ok(_x) => println!("Opening the Nvidia Settings app.."),
            Err(y) => println!(
                "An error occured while opening the Nvidia Settings app: {}",
                y.message()
            ),
        };
    }

    fn task5(&self) {
        match subprocess::exec_check(&[OsStr::new("nvidia-settings")], None::<&gio::Cancellable>) {
            Ok(_x) => println!("Opening the Nvidia Settings app.."),
            Err(y) => println!(
                "An error occured while opening the Nvidia Settings app: {}",
                y.message()
            ),
        };
    }

    fn setup_actions(&self) {
        // Create action from key "task1" and add to action group "win"
        let task1 = self.settings().create_action("task1");
        self.add_action(&task1);

        // Create action from key "task2" and add to action group "win"
        let task2 = self.settings().create_action("task2");
        self.add_action(&task2);

        // Create action from key "task3" and add to action group "win"
        let task3 = self.settings().create_action("task3");
        self.add_action(&task3);


        // Create action from key "task4" and add to action group "win"
        let task4 = self.settings().create_action("task4");
        self.add_action(&task4);


        // Create action from key "task5" and add to action group "win"
        let task5 = self.settings().create_action("task5");
        self.add_action(&task5);


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
