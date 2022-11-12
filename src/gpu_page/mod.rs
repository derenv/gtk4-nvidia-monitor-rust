// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (GpuPage)
 *
 * Made:
 * 02/11/2022
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
use glib::Object;
use gio::Settings;
use adwaita::{glib, gio};
use gtk::{subclass::prelude::*, prelude::*, Label, Orientation::Horizontal, Box, LayoutChild, Button};
use std::{sync::MutexGuard, sync::Arc, sync::Mutex};

// Modules
use crate::{APP_ID, provider::Provider};

// GObject wrapper for GpuPage
glib::wrapper! {
    pub struct GpuPage(ObjectSubclass<imp::GpuPage>)
    @extends gtk::Grid, gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

/**
 * Trait Name:
 * GpuPage
 *
 * Description:
 * Trait shared by all properties
 *
 * Made:
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl GpuPage {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new GpuPage object
     *
     * Made:
     * 02/11/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn new(uuid: &str, name: &str, provider: &Provider) -> Self {
        let obj: GpuPage = Object::new(&[]).expect("Failed to create `GpuPage`.");

        // Set custom properties
        obj.set_property("uuid", String::from(uuid));
        obj.set_property("name", String::from(name));
        obj.set_property("provider", provider);

        // Apply any setup actions that need the above properties
        obj.setup_settings();
        obj.setup_widgets();

        obj
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
     * <https://stackoverflow.com/questions/70986004/gtk-rs-set-label-within-glibtimeout-add>
     * <https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html>
     */
    fn setup_widgets(&self) {
        // Fetch grid's layout manager
        match self.layout_manager() {
            Some(grid_manager) => {
                // Get reference to settings object
                let settings_obj: &Settings = self.settings();

                // Load refresh time (s) from settings
                let refresh_rate: u32 = settings_obj.get::<i32>("refreshrate") as u32;

                // List of used statistics
                // TODO: Load config of gpu (use uuid as ID)
                // TODO: needs to be a vector as may bed of variable size..
                //let statistics: Vec<&str> = load_json_settings(&self.property("uuid"));//array?vector?json-type object?
                let statistics_data: Vec<&str> = vec![
                    "util",
                    "temp",
                    "memory_usage",
                    "memory_total",
                    "fan_speed",
                    "power_usage"
                ];
                let statistics_store: Arc<Mutex<Vec<&str>>> = Arc::new(Mutex::new(statistics_data));

                // Edit button
                let edit_button: Button = Button::builder()
                    .build();
                self.attach(&edit_button, 3, 0, 24, 24);

                // Set layout properties of button
                let child_manager: LayoutChild = grid_manager.layout_child(&edit_button);
                child_manager.set_property("row-span", 2);
                child_manager.set_property("column-span", 2);


                // For each Statistic
                let mut labels: Vec<Label> = Vec::new();
                for statistic in Arc::clone(&statistics_store).lock().unwrap().iter() {
                    //==BUILD==
                    // Build label & add to grid
                    let new_title: String = String::from(statistic.to_owned()) + "_label";
                    let new_title_label: Label = Label::builder()
                        .label(statistic)
                        .name(&new_title)
                        .build();

                    // Build label & add to grid
                    let new_content: String = String::from(statistic.to_owned());
                    let new_content_label: Label = Label::builder()
                        .label("")
                        .name(&new_content)
                        .build();

                    // Create box for 2 labels
                    let new_box_name: String = String::from(statistic.to_owned()) + "_box";
                    let new_box: Box = Box::builder()
                        .name(&new_box_name)
                        .orientation(Horizontal)
                        .build();
                    new_box.append(&new_title_label);
                    new_box.append(&new_content_label);
                    self.attach(&new_box, 0, labels.len() as i32, 24, 24);

                    // Set layout properties of box
                    let child_manager: LayoutChild = grid_manager.layout_child(&new_box);
                    child_manager.set_property("row-span", 2);
                    child_manager.set_property("column-span", 2);


                    //==SHOW==
                    // Show new labels & box
                    new_box.show();
                    new_title_label.show();
                    new_content_label.show();

                    // Add to list of content labels, for updating in closure (see below)
                    labels.push(new_content_label);
                }

                // Fetch uuid, needed for processor
                let uuid_store: Arc<Mutex<String>> = Arc::new(Mutex::new(self.property("uuid")));

                // Async fill the labels
                glib::timeout_add_seconds_local(refresh_rate, move || {
                    // Grab locked data
                    // Get list of statistics
                    let statistics_lock: Arc<Mutex<Vec<&str>>> = Arc::clone(&statistics_store);
                    let statistics: MutexGuard<Vec<&str>> = statistics_lock.lock().unwrap();
                    // Get uuid
                    let uuid_lock: Arc<Mutex<String>> = Arc::clone(&uuid_store);
                    let uuid: String = uuid_lock.lock().unwrap().as_str().to_owned();

                    // Create provider for scanning gpu data
                    let provider: Provider = Provider::new(
                        || {
                            vec![]
                        },
                        0,
                    );

                    // For each Statistic
                    for statistic in statistics.iter() {
                        // Grab current stat from provider
                        match provider.get_gpu_data(&uuid, statistic) {
                            Ok(stat) => {
                                // For each output label of the page
                                for label in &labels {
                                    // Check if correct label
                                    if String::from(statistic.to_owned()) == String::from(label.widget_name()) {
                                        label.set_label(&stat);
                                    }
                                }
                            }
                            Err(err) => {
                                println!("panicked when fetching gpu data: `{}`", err);
                                return Continue(false)
                            }
                        }
                    }

                    return Continue(true)
                });
            },
            None => panic!("Cannot fetch layout manager of grid.."),
        }
    }

    /**
     * Name:
     * restore_data
     *
     * Description:
     * Restore properties from settings
     *
     * Made:
     * 07/11/2022
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
}

/**
 * Trait Name:
 * Default
 *
 * Description:
 * Default object
 *
 * Made:
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl Default for GpuPage {
    fn default() -> Self {
        Self::new("", "Default", &Provider::default())
    }
}
