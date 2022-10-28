// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (MainWindow)
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

// Imports
use adwaita::{gio, glib, prelude::*, subclass::prelude::*, ActionRow};
use gio::Settings;
use glib::{once_cell::sync::OnceCell, signal::Inhibit, subclass::InitializingObject};
use gtk::{subclass::prelude::*, CompositeTemplate, ListBox, TemplateChild};
use std::{cell::Cell, cell::RefCell, rc::Rc};

use crate::formatter::Formatter;
use crate::processor::Processor;
use crate::property::Property;
use crate::provider::Provider;
// Modules
//use crate::utils::data_path;
use crate::custom_button::CustomButton;
use crate::settingswindow::SettingsWindow;

// Structure for storing SettingsWindow and info
#[derive(Default)]
pub struct SettingsWindowContainer {
    pub window: Option<SettingsWindow>,
    pub open: bool,
}

// Object holding the State
#[derive(CompositeTemplate, Default)]
#[template(resource = "/main-window.ui")]
pub struct MainWindow {
    pub settings: OnceCell<Settings>,
    pub app_id: Cell<String>,
    pub settings_window: Rc<RefCell<SettingsWindowContainer>>,
    pub provider: Cell<Option<Provider>>,

    #[template_child]
    pub cards_list: TemplateChild<ListBox>,

    #[template_child]
    pub refresh_button: TemplateChild<CustomButton>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NvidiaExtensionMainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

/*
 * Name:
 * MainWindow
 *
 * Description:
 * Trait shared by all MainWindow objects
 *
 * Made:
 * 13/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
#[gtk::template_callbacks]
impl MainWindow {
    #[template_callback]
    fn card_selected(&self, row: &ActionRow) {
        println!("CARD CHOSEN: {}", row.title());
    }

    pub fn create_provider(provider_type: i32) -> Provider {
        match provider_type {
            0 => {
                // Nvidia Settings and Nvidia SMI
                // Create new provider
                Provider::new(
                    || {
                        vec![
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "utilization.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "temperature.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "memory.used,memory.total",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "fan.speed",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "power.draw",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                        ]
                    },
                    0,
                )
            }
            1 => {
                // Nvidia Settings
                // Create new provider
                Provider::new(
                    || {
                        vec![
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "utilization.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "temperature.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "memory.used,memory.total",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "fan.speed",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                        ]
                    },
                    1,
                )
            }
            2 => {
                // Nvidia SMI
                // Create new provider
                Provider::new(
                    || {
                        vec![
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "utilization.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "temperature.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "memory.used,memory.total",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "fan.speed",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "power.draw",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                        ]
                    },
                    2,
                )
            }
            3 => {
                // Nvidia Optimus
                // Create new provider
                Provider::new(
                    || {
                        vec![
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "utilization.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "temperature.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "memory.used,memory.total",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "fan.speed",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "power.draw",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                        ]
                    },
                    3,
                )
            }
            _ => {
                // Assume Default (Nvidia Settings and Nvidia SMI)
                // Create new provider
                Provider::new(
                    || {
                        vec![
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "utilization.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "temperature.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "memory.used,memory.total",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "fan.speed",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=gpu_name --format=csv,noheader",
                                ),
                                "power.draw",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                        ]
                    },
                    0,
                )
            }
        }
    }

    #[template_callback]
    fn refresh_cards(&self, button: &CustomButton) {
        //TEST: Grab button label
        let label_val = button.label().expect("cannot grab label of refresh button");
        println!("Button Pressed: {}", label_val);

        // Clear current ActionRow objects from GtkListBox
        let mut done: bool = false;
        while !done {
            // Try to grab a child
            let current_child: Option<gtk::Widget> = self.cards_list.get().first_child();

            // Check if there are any children left
            match current_child {
                Some(valid_child) => {
                    // Remove child
                    self.cards_list.get().remove(&valid_child);
                }
                None => {
                    // End loop
                    done = true;
                }
            }
        }

        // Grab settings
        let settings: &Settings = self.settings.get().expect("..Cannot retrieve settings");

        // Grab provider
        let provider: Option<Provider> = self.provider.take();

        // If provider does not exist
        match provider {
            Some(existing_provider) => {
                // Update GPU list
                //let gpu_uuids = existing_provider.gpus;
                let gpu_uuids = ["", ""];

                // Store GPU list in settings
                settings
                    .set_strv("gpus", &gpu_uuids)
                    .expect("..Cannot store list of GPU UUID in `gpus` setting");

                // Re-Store provider
                self.provider.set(Some(existing_provider));
            }
            None => {
                // Check provider type
                let provider_type: i32 = self
                    .settings
                    .get()
                    .expect("..cannot fetch settings")
                    .int("provider");

                let new_provider: Provider = MainWindow::create_provider(provider_type);

                // Update GPU list
                //let gpu_uuids = new_provider.gpus;
                let gpu_uuids = ["", ""];

                // Store new provider
                self.provider.set(Some(new_provider));

                // Store GPU list in settings
                settings
                    .set_strv("gpus", &gpu_uuids)
                    .expect("..Cannot store list of GPU UUID in `gpus` setting");
            }
        }

        // Grab provider
        let provider: Option<Provider> = self.provider.take();

        // Fetch updated GPU list
        //let gpus: Vec<GString> = settings.strv("gpus");

        // For each scanned GPU
        let gpus = vec![
            ("title 1", "subtitle 1"),
            ("title 2", "subtitle 2"),
            ("title 3", "subtitle 3"),
        ]; //TEST
        for i in 0..gpus.len() {
            // Get GPU data
            let title: &str = gpus[i].0; //TEST
            let subtitle: &str = gpus[i].1; //TEST
                                            //let gpu_name = provider.get_gpu_data(&gpus[i], "name");
                                            //let gpu_uuid = provider.get_gpu_data(&gpus[i], "UUID");

            // Create new ActionRow object
            let current_row: ActionRow =
                ActionRow::builder().title(title).subtitle(subtitle).build();

            // Append new ActionRow object to GtkListBox
            self.cards_list.append(&current_row);
        }

        // Put provider back
        self.provider.set(provider);
    }
}
/*
 * Trait Name:
 * ObjectImpl
 *
 * Description:
 * Trait shared by all GObjects
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
impl ObjectImpl for MainWindow {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        // Setup
        obj.setup_settings();
        obj.setup_widgets();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_actions();
    }
}

/*
 * Trait Name:
 * WidgetImpl
 *
 * Description:
 * Trait shared by all widgets
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
impl WidgetImpl for MainWindow {}

/*
 * Trait Name:
 * WindowImpl
 *
 * Description:
 * Trait shared by all Window's
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
impl WindowImpl for MainWindow {
    fn close_request(&self, window: &Self::Type) -> Inhibit {
        /*
        // Store task data in vector
        let backup_data: Vec<TaskData> = window
            .tasks()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<TaskObject>)
            .map(TaskObject::task_data)
            .collect();

        // Save state to file
        let file = File::create(data_path()).expect("Could not create json file.");
        serde_json::to_writer(file, &backup_data)
            .expect("Could not write data to json file");

        */
        // Set state in settings
        let settings: &Settings = window.settings();
        settings
            .set_boolean("app-settings-open", false)
            .expect("Could not set setting.");
        settings
            .set_boolean("nvidia-settings-open", false)
            .expect("Could not set setting.");

        // Pass close request on to the parent
        self.parent_close_request(window)
    }
}

/*
 * Trait Name:
 * AdwWindowImpl
 *
 * Description:
 * Trait shared by all AdwWindow's
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
impl AdwWindowImpl for MainWindow {}

/*
 * Trait Name:
 * ApplicationWindowImpl
 *
 * Description:
 * Trait shared by all ApplicationWindow's
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
impl ApplicationWindowImpl for MainWindow {}

/*
 * Trait Name:
 * AdwApplicationWindowImpl
 *
 * Description:
 * Trait shared by all AdwApplicationWindow's
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
impl AdwApplicationWindowImpl for MainWindow {}
