// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
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
use glib::{once_cell::sync::OnceCell, signal::Inhibit, subclass::InitializingObject, FromVariant, once_cell::sync::Lazy, ParamSpec, Value};
use gtk::{subclass::prelude::*, CompositeTemplate, ListBox, TemplateChild};
use std::{cell::Cell, cell::RefCell, rc::Rc};

// Modules
use crate::{custom_button::CustomButton, settingswindow::SettingsWindow, formatter::Formatter, processor::Processor, property::Property, provider::Provider};

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
    pub settings_window: Rc<RefCell<SettingsWindowContainer>>,
    pub provider: Cell<Option<Provider>>,

    //pub pages: Cell<Vec<GtkBox>>,

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

/**
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
    /**
     * Name:
     * update_setting
     *
     * Description:
     * Generic function for updating setting values
     *
     * Made:
     * 30/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn update_setting<T: ToVariant>(&self, name: &str, value: T) {
        // Fetch settings
        match self.settings.get() {
            Some(settings) => {
                match settings.set(name, &value) {
                    Ok(_) => println!("..Setting `{}` updated!", name),
                    Err(err) => panic!("..Cannot update `{}` setting: `{}`", name, err),
                }
            },
            None => panic!("..Cannot retrieve settings")
        }
    }

    /**
     * Name:
     * get_setting
     *
     * Description:
     * Generic function for getting setting value
     *
     * Made:
     * 30/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn get_setting<T: FromVariant>(&self, name: &str) -> T {
        // Return the value of the property
        match self.settings.get() {
            Some(settings) => {
                settings.get::<T>(name)
            },
            None => panic!("`settings` should be set in `setup_settings`.")
        }
    }

    /**
     * Name:
     * card_selected
     *
     * Description:
     * Template callback for GPU card/row item selection
     *
     * Made:
     * 28/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * (placeholder, needs updated when right hand pane is working)
     */
    #[template_callback]
    fn card_selected(&self, row: &ActionRow) {
        println!("CARD CHOSEN: {}", row.title());//TEST

        /*
        // Activate appropriate page of right pane
        // For each of the currently stored pages
        let exists: bool = false;
        for page in self.pages.take() {
            // Check if the page exists
            if page.uuid == row.subtitle() {
                exists = true;

                // Set current_page
                //set current_page to page(row.subtitle())
            }
        }
        if !exists {
        //    // Create new page using UUID
        //    create new_page using row.subtitle()
        //
        //    // Add new page to list of currently stored pages
        //    add new_page to self.pages
        //
        //    // Set current_page
        //    set current_page to new_page
        }
        */
    }

    /**
     * Name:
     * create_provider
     *
     * Description:
     * Creates a provider object of a certain type (given as input parameter)
     *
     * Made:
     * 28/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * ??causes crash??
     */
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
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "temperature.gpu",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "memory.used,memory.total",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "fan.speed",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "power.draw",
                                "",
                                &Formatter::new(),
                                &0,
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
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "temperature.gpu",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "memory.used,memory.total",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "fan.speed",
                                "",
                                &Formatter::new(),
                                &0,
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
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "utilization.gpu",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "temperature.gpu",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "memory.used,memory.total",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "fan.speed",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "power.draw",
                                "",
                                &Formatter::new(),
                                &0,
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
                                &Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"),
                                "utilization.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"),
                                "temperature.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"),
                                "memory.used,memory.total",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"),
                                "fan.speed",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"),
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
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "utilization.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "temperature.gpu",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "memory.used,memory.total",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "fan.speed",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
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



    /**
     * Name:
     * refresh_cards
     *
     * Description:
     * Template callback for GPU list refresh button
     *
     * Made:
     * 28/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    #[template_callback]
    fn refresh_cards(&self, button: &CustomButton) {
        //TEST
        match button.label() {
            Some(label_val) => println!("Button Pressed: {}", label_val),
            None => panic!("..Cannot grab label of refresh button")
        }

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

        // Grab provider
        let provider: Option<Provider> = self.provider.take();

        // If provider does not exist
        match provider {
            Some(existing_provider) => {
                // Update GPU list
                match existing_provider.get_gpu_uuids() {
                    Ok(gpu_uuids) => {
                        // Get GPU list
                        let gpu_count: i32 = gpu_uuids.len() as i32;

                        // Update each property
                        match existing_provider.update_property_value::<i32>("gpu-count", gpu_count) {
                            Ok(_) => {
                                // Construct a row for each GPU
                                for uuid in gpu_uuids {
                                    // Get GPU data
                                    match existing_provider.get_gpu_data(&uuid, "name") {
                                        Ok(gpu_name) => {
                                            // Create new ActionRow object
                                            let current_row: ActionRow =
                                            ActionRow::builder()
                                                .title(&gpu_name)
                                                .subtitle(&uuid)
                                                .activatable(true)
                                                .selectable(true)
                                                .build();

                                            // Append new ActionRow object to GtkListBox
                                            self.cards_list.append(&current_row);
                                        },
                                        Err(err) => {
                                            println!("..Attempt to read GPU name failed, returning: {}", err);

                                            // Create new ActionRow object
                                            let current_row: ActionRow =
                                            ActionRow::builder()
                                                .title(&uuid)
                                                .subtitle(&uuid)
                                                .activatable(true)
                                                .selectable(true)
                                                .build();

                                            // Append new ActionRow object to GtkListBox
                                            self.cards_list.append(&current_row);
                                        }
                                    }
                                }
                            }
                            Err(err) => println!("..Attempt to read GPU data failed, returning: {}", err),
                        }
                    }
                    Err(err) => println!("..Attempt to update GPU list failed, returning: {}", err),
                }

                // Re-Store provider
                self.provider.set(Some(existing_provider));
            }
            None => {
                // Check provider type
                let provider_type: i32 = self.get_setting::<i32>("provider");

                let new_provider: Provider = MainWindow::create_provider(provider_type);

                // Update GPU list
                match new_provider.get_gpu_uuids() {
                    Ok(gpu_uuids) => {
                        // Get GPU list
                        let gpu_count: i32 = gpu_uuids.len() as i32;

                        // Update each property
                        match new_provider.update_property_value::<i32>("gpu-count", gpu_count) {
                            Ok(_) => {
                                // Construct a row for each GPU
                                for uuid in gpu_uuids {
                                    // Get GPU data
                                    match new_provider.get_gpu_data(&uuid, "name") {
                                        Ok(gpu_name) => {
                                            // Create new ActionRow object
                                            let current_row: ActionRow =
                                            ActionRow::builder()
                                                .title(&gpu_name)
                                                .subtitle(&uuid)
                                                .activatable(true)
                                                .selectable(true)
                                                .build();

                                            // Append new ActionRow object to GtkListBox
                                            self.cards_list.append(&current_row);
                                        },
                                        Err(err) => {
                                            println!("..Attempt to read GPU name failed, returning: {}", err);

                                            // Create new ActionRow object
                                            let current_row: ActionRow =
                                            ActionRow::builder()
                                                .title(&uuid)
                                                .subtitle(&uuid)
                                                .activatable(true)
                                                .selectable(true)
                                                .build();

                                            // Append new ActionRow object to GtkListBox
                                            self.cards_list.append(&current_row);
                                        }
                                    }
                                }
                            }
                            Err(err) => println!("..Attempt to read GPU data failed, returning: {}", err),
                        }
                    }
                    Err(err) => println!("..Attempt to update GPU list failed, returning: {}", err),
                }

                // Store new provider
                self.provider.set(Some(new_provider));
            }
        }
    }
}
/**
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
    /**
     * Name:
     * constructed
     *
     * Description:
     * Called during construction, allows calling setup functions
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

    /**
     * Name:
     * properties
     *
     * Description:
     * Create list of custom properties for our GObject
     *
     * Made:
     * 06/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * beware that you need to use kebab-case (<https://en.wikipedia.org/wiki/Letter_case#Kebab_case>)
     *
     * ParamSpec Examples:
     * glib::ParamSpecString::builder("icon").build(),
     * glib::ParamSpecUInt::builder("gpu_count").build(),
     * glib::ParamSpecString::builder("call_extension").build(),
     * TODO: these are from property class
     * glib::ParamSpecBoxed::builder("processor").build(),
     * glib::ParamSpecObject::builder("formatter").build(),
     */
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecObject::builder("provider", glib::Type::OBJECT).build(),
            ]
        });

        //println!("PROPERTIES: {:?}", PROPERTIES);//TEST
        //println!("trying to add `base_call`: {:?}", glib::ParamSpecString::builder("base_call").build());//TEST

        PROPERTIES.as_ref()
    }

    /**
     * Name:
     * set_property
     *
     * Description:
     * Mutator for custom GObject properties
     *
     * Made:
     * 06/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        //println!("setting: {:?}", pspec.name());//TEST

        match pspec.name() {
            "provider" => {
                let input_provider_property: Option<Provider> = value
                    .get()
                    .expect("The value needs to be of type `Provider`.");
                self.provider.replace(input_provider_property);
            }
            _ => panic!("Property `{}` does not exist..", pspec.name())
        }
    }

    /**
     * Name:
     * property
     *
     * Description:
     * Accessor for custom GObject properties
     *
     * Made:
     * 06/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        //println!("getting: {:?}", pspec.name());//TEST

        match pspec.name() {
            "provider" => {
                //TODO: this seems ridiculous..
                let value: Option<Provider> = self.provider.take();

                self.provider.set(value.clone());

                value.to_value()
            }
            _ => panic!("Property `{}` does not exist..", pspec.name())
        }
    }
}

/**
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

/**
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
        self.update_setting("app-settings-open", false);
        self.update_setting("nvidia-settings-open", false);

        // Pass close request on to the parent
        self.parent_close_request(window)
    }
}

/**
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

/**
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

/**
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
