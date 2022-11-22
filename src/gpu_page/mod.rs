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
use adwaita::{gio, glib};
use gio::Settings;
use glib::Object;
use gtk::{prelude::*, subclass::prelude::*, Align, Button, Grid, Label, LayoutChild, Orientation};
use std::{sync::Arc, sync::Mutex, sync::MutexGuard};

// Modules
use crate::{provider::Provider, APP_ID};

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
    pub fn new(uuid: &str, name: &str, provider: Provider) -> Self {
        let obj: GpuPage = Object::new(&[]).expect("Failed to create `GpuPage`.");

        // Set custom properties
        obj.set_property("uuid", String::from(uuid));
        obj.set_property("name", String::from(name));
        obj.set_property("provider", &provider);

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
                let statistics_data: Vec<&str>; //TEST
                match self
                    .property::<Provider>("provider")
                    .property::<i32>("provider-type")
                {
                    1 => {
                        statistics_data = vec![
                            "util",
                            "temp",
                            "memory_usage",
                            "memory_total",
                            "mem_ctrl_util",
                        ];
                    }
                    _ => {
                        statistics_data = vec![
                            "util",
                            "temp",
                            "memory_usage",
                            "memory_total",
                            "mem_ctrl_util",
                            "fan_speed",
                            "power_usage",
                        ];
                    }
                }
                let statistics_store: Arc<Mutex<Vec<&str>>> = Arc::new(Mutex::new(statistics_data));

                // Edit button
                let edit_button: Button = Button::builder()
                    .name("edit_button")
                    .label("Add Statistic")
                    .margin_start(12)
                    .margin_end(12)
                    .margin_top(12)
                    .margin_bottom(12)
                    .build();
                self.attach(&edit_button, 4, 0, 24, 24);

                // Set layout properties of button
                let child_manager: LayoutChild = grid_manager.layout_child(&edit_button);
                child_manager.set_property("row-span", 2);
                child_manager.set_property("column-span", 2);

                // For each Statistic
                let mut labels: Vec<Label> = Vec::new();
                for statistic in Arc::clone(&statistics_store).lock().unwrap().iter() {
                    //==BUILD==
                    // Build grid for 2 labels and attach to this page
                    //let new_grid_name: String = String::from(statistic.to_owned()) + "_grid";
                    let new_grid_name: String = String::from("statistic_item_grid");
                    let new_grid: Grid = Grid::builder()
                        .name(&new_grid_name)
                        .orientation(Orientation::Horizontal)
                        //.margin_start(12)
                        //.margin_end(12)
                        //.margin_top(12)
                        //.margin_bottom(12)
                        //.halign(Align::Center)
                        //.valign(Align::Center)
                        //.hexpand(true)
                        //.vexpand(true)
                        .build();
                    self.attach(&new_grid, 0, labels.len() as i32, 100, 12);

                    // Set layout properties of grid
                    let child_manager: LayoutChild = grid_manager.layout_child(&new_grid);
                    child_manager.set_property("row-span", 1);
                    child_manager.set_property("column-span", 1);
                    //child_manager.set_property("outline-style", "solid");
                    //child_manager.set_property("outline-width", 1);
                    //child_manager.set_property("border-radius", 3);

                    // Fetch layout manager for this (grid) child
                    let internal_grid_manager = new_grid.layout_manager().expect("Fuck..");

                    // Decide on title label size
                    let space: i32;
                    let pretty_label: &str;
                    match statistic.to_owned() {
                        "util" => {
                            pretty_label = "GPU Utilization";
                            space = 5
                        }
                        "mem_ctrl_util" => {
                            pretty_label = "Memory Controller Utilization";
                            space = 5
                        }
                        "encoder_util" => {
                            pretty_label = "Encoder Utilization";
                            space = 5
                        }
                        "decoder_util" => {
                            pretty_label = "Decoder Utilization";
                            space = 5
                        }
                        "fan_speed" => {
                            pretty_label = "Fan Speed";
                            space = 5
                        }
                        "temp" => {
                            pretty_label = "Temperature";
                            space = 5
                        }
                        "memory_usage" => {
                            pretty_label = "Memory Usage";
                            space = 8
                        }
                        "memory_total" => {
                            pretty_label = "Memory Total";
                            space = 8
                        }
                        "power_usage" => {
                            pretty_label = "Power Usage";
                            space = 8
                        }
                        _ => {
                            pretty_label = statistic;
                            space = 5
                        }
                    }

                    // Build title label & add to grid
                    let new_title: String = String::from(statistic.to_owned()) + "_label";
                    let new_title_label: Label = Label::builder()
                        .label(pretty_label)
                        .name(&new_title)
                        .hexpand(true)
                        .hexpand_set(true)
                        .halign(Align::Center)
                        //.valign(Align::Center)
                        .margin_top(12)
                        .margin_bottom(12)
                        .width_chars(space)
                        .build();
                    new_grid.attach(&new_title_label, 0, 0, 1, 1);

                    // Set layout properties of (title label) child
                    let title_manager: LayoutChild =
                        internal_grid_manager.layout_child(&new_title_label);
                    title_manager.set_property("row-span", 1);

                    // Decide on content label size
                    let space: i32;
                    match statistic.to_owned() {
                        "util" | "fan_speed" | "temp" => space = 5,
                        "memory_usage" | "memory_total" => space = 8,
                        _ => space = 5,
                    }

                    // Build content label & add to grid
                    let new_content: String = String::from(statistic.to_owned());
                    let new_content_label: Label = Label::builder()
                        .label("")
                        .name(&new_content)
                        //.halign(Align::End)
                        //.valign(Align::Center)
                        .width_chars(space)
                        .build();
                    new_grid.attach(&new_content_label, 1, 0, 1, 1);

                    // Set layout properties of (content label) child
                    let content_manager: LayoutChild =
                        internal_grid_manager.layout_child(&new_content_label);
                    content_manager.set_property("row-span", 1);

                    //==SHOW==
                    // Show new labels & grid
                    new_grid.show();
                    new_title_label.show();
                    new_content_label.show();

                    // Add to list of content labels, for updating in closure (see below)
                    labels.push(new_content_label);
                }

                // Create thread safe container for uuid, needed for processor
                let uuid_store: Arc<Mutex<String>> = Arc::new(Mutex::new(self.property("uuid")));

                // Create thread safe container for provider
                let provider_store: Arc<Mutex<Option<Provider>>> =
                    Arc::new(Mutex::new(self.property("provider")));

                // Async fill the labels
                glib::timeout_add_seconds_local(refresh_rate, move || {
                    // Grab locked data
                    // list of statistics
                    let statistics_lock: Arc<Mutex<Vec<&str>>> = Arc::clone(&statistics_store);
                    let statistics: MutexGuard<Vec<&str>> = statistics_lock.lock().unwrap();
                    // uuid
                    let uuid_lock: Arc<Mutex<String>> = Arc::clone(&uuid_store);
                    let uuid: String = uuid_lock.lock().unwrap().as_str().to_owned();
                    // current provider for scanning gpu data
                    let provider_lock: Arc<Mutex<Option<Provider>>> = Arc::clone(&provider_store);
                    let mut provider_container: MutexGuard<Option<Provider>> =
                        provider_lock.lock().unwrap();

                    // For each Statistic
                    match &mut *provider_container {
                        Some(prov) => {
                            for statistic in statistics.iter() {
                                // Grab current stat from provider
                                match prov.get_gpu_data(&uuid, statistic) {
                                    Ok(stat) => {
                                        // For each output label of the page
                                        for label in &labels {
                                            // Check if correct label
                                            if *statistic.to_owned() == label.widget_name() {
                                                label.set_label(&stat);
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        println!("panicked when fetching gpu data: `{}`", err);
                                        return Continue(false);
                                    }
                                }
                            }
                        }
                        None => todo!(),
                    }

                    Continue(true)
                });
            }
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
        Self::new("", "Default", Provider::default())
    }
}
