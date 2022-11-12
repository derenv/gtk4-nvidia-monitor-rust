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
use adwaita::{gio, glib, prelude::*, subclass::prelude::*};
use gio::Settings;
use glib::{once_cell::sync::OnceCell, signal::Inhibit, subclass::InitializingObject, FromVariant, once_cell::sync::Lazy, ParamSpec, Value};
use gtk::{subclass::prelude::*, CompositeTemplate, Stack, TemplateChild, Button, ScrolledWindow, PolicyType};
use std::{cell::Cell, cell::RefCell, rc::Rc, cell::RefMut};

// Modules
use crate::{settingswindow::SettingsWindow, formatter::Formatter, processor::Processor, property::Property, provider::Provider, gpu_page::GpuPage};

/// Structure for storing a SettingsWindow object and any related information
#[derive(Default)]
pub struct SettingsWindowContainer {
    pub window: Option<SettingsWindow>,
    pub open: bool,
}

/// Object holding the State and any Template Children
#[derive(CompositeTemplate, Default)]
#[template(resource = "/main-window.ui")]
pub struct MainWindow {
    pub settings: OnceCell<Settings>,
    pub settings_window: Rc<RefCell<SettingsWindowContainer>>,
    pub provider: Cell<Option<Provider>>,

    gpu_pages: RefCell<Vec<GpuPage>>,

    #[template_child]
    pub gpu_stack: TemplateChild<Stack>,
}

/// The central trait for subclassing a GObject
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
     *
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
                                "memory.used",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "memory.total",
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
                                "memory.used",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q GpuUUID -t"),
                                "memory.total",
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
                                "memory.used",
                                "",
                                &Formatter::new(),
                                &0,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "memory.total",
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
                                "memory.used",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"),
                                "memory.total",
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
                                "memory.used",
                                "",
                                &Formatter::new(),
                                &1,
                            ),
                            Property::new(
                                &Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader"),
                                "memory.total",
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

    /*
     * Name:
     * create_gpu_page
     *
     * Description:
     * Create a new object of type GpuPage and add to stack
     *
     * Made:
     * 07/11/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn create_gpu_page(&self, uuid: &str, name: &str, provider: &Provider) {
        // Create new GpuPage object
        let new_page: GpuPage = GpuPage::new(uuid, name, provider);

        // Add to list of pages
        let mut gpu_page_list: RefMut<Vec<GpuPage>> = self.gpu_pages.borrow_mut();
        gpu_page_list.push(new_page);
        match gpu_page_list.last() {
            Some(new_page_ref) => {
                // Create scrollable container
                let scrolled_window: ScrolledWindow = ScrolledWindow::new();
                scrolled_window.set_hscrollbar_policy(PolicyType::Never);
                scrolled_window.set_vscrollbar_policy(PolicyType::Automatic);
                scrolled_window.set_child(Some(new_page_ref));

                // Append new ListBoxRow object to GtkListBox
                self.gpu_stack.add_titled(&scrolled_window, Some(uuid), name);
            },
            None => panic!("COULD NOT FETCH GPU PAGE REF"),
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
    fn refresh_cards(&self, _button: &Button) {
        println!("GPU Scan Button Pressed!");//TEST

        // Clear current ActionRow objects from GtkListBox
        let mut done: bool = false;
        while !done {
            // Try to grab a child
            let current_child: Option<gtk::Widget> = self.gpu_stack.get().first_child();

            // Check if there are any children left
            match current_child {
                Some(valid_child) => {
                    // Remove child
                    self.gpu_stack.get().remove(&valid_child);
                }
                None => {
                    // End loop
                    done = true;
                }
            }
        }

        // Grab copy of current provider
        let provider: Option<Provider> = self.provider.take();
        self.provider.set(provider.clone());

        // If provider does not exist
        match provider {
            Some(existing_provider) => {
                // Update GPU list
                match existing_provider.get_gpu_uuids() {
                    Ok(gpu_uuids) => {
                        // Update each property
                        match existing_provider.update_property_value::<i32>("gpu-count", gpu_uuids.len() as i32) {
                            Ok(_) => {
                                // Construct a row for each GPU
                                for uuid in gpu_uuids {
                                    // Get GPU data
                                    match existing_provider.get_gpu_data(&uuid, "name") {
                                        Ok(gpu_name) => {
                                            // Create new GpuPage object and Add to list of pages
                                            self.create_gpu_page(&uuid, &gpu_name, &existing_provider);
                                        },
                                        Err(err) => {
                                            println!("..Attempt to read GPU name failed, returning: {}", err);

                                            // Create new GpuPage object and Add to list of pages
                                            self.create_gpu_page(&uuid, &uuid, &existing_provider);
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
                //self.provider.set(Some(existing_provider));
            }
            None => {
                // Check provider type
                let provider_type: i32 = self.get_setting::<i32>("provider");

                // Create new provider
                let new_provider: Provider = MainWindow::create_provider(provider_type);

                // Update GPU list
                match new_provider.get_gpu_uuids() {
                    Ok(gpu_uuids) => {
                        // Get GPU list size
                        let gpu_count: i32 = gpu_uuids.len() as i32;

                        // Update each property
                        match new_provider.update_property_value::<i32>("gpu-count", gpu_count) {
                            Ok(_) => {
                                // Construct a row for each GPU
                                for uuid in gpu_uuids {
                                    // Get GPU data
                                    match new_provider.get_gpu_data(&uuid, "name") {
                                        Ok(gpu_name) => {
                                            // Create new GpuPage object and Add to list of pages
                                            self.create_gpu_page(&uuid, &gpu_name, &new_provider);
                                        },
                                        Err(err) => {
                                            println!("..Attempt to read GPU name failed, returning: {}", err);

                                            // Create new GpuPage object and Add to list of pages
                                            self.create_gpu_page(&uuid, &uuid, &new_provider);
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
                //self.provider.set(Some(new_provider));
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
                match value.get() {
                    Ok(input_provider_property) => {
                        self.provider.replace(input_provider_property);
                    },
                    Err(_) => panic!("The value needs to be of type `Provider`."),
                }
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
    /*
     * Name:
     * close_request
     *
     * Description:
     * Run when window closed
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
    fn close_request(&self, window: &Self::Type) -> Inhibit {
        // Create vector for final saving
        let mut save_data: Vec<Vec<(String, String)>> = vec![];
        //let mut save_data: gio::ListStore;

        // For each GPU page
        for current_page in self.gpu_pages.take() {
            // Grab data stored in page
            let uuid: String = current_page.property::<String>("uuid");
            //println!("UUID FETCH: {}", current_page.property::<String>("uuid"));
            let name: String = current_page.property::<String>("name");
            //println!("NAME FETCH: {}", current_page.property::<String>("name"));

            // Create vector for current GPU
            let mut current_stats: Vec<(String, String)> = vec![];

            // For stat object in page
            //for label in current_page.property::<>("stats") {
                //
            //}
            // PLACEHOLDER
            current_stats.push(("uuid".to_string(), uuid));
            current_stats.push(("name".to_string(), name));
            // PLACEHOLDER

            // Add current vector to final vector
            save_data.push(current_stats);
            //let test_s = gtk::Label::new(Some("FUCK"));
            //save_data.append(&test_s);
        }

        // Create collection to store in json
        // TEST OUTPUT
        for gpu in save_data {
            for stat in gpu {
                println!("STAT NAME: `{}` STAT VALUE: `{}`", stat.0, stat.1);
            }
        }
        // let qq = save_data
        //      .snapshot() //?
        //      .iter()
        //      .filter_map(Cast::downcast_ref::<TaskObject>)
        //      .map(TaskObject::task_data)
        //      .collect();

        // let gpu_data: Vec<TaskData> = current_page
        //     .tasks()    // change to "items" and grab all statistic pairs/objects
        //     .snapshot() //?
        //     .iter()
        //     .filter_map(Cast::downcast_ref::<TaskObject>)
        //     .map(TaskObject::task_data)
        //     .collect();

        // Store in json
        // Create json file object
        //let file = File::create(data_path()).expect("Could not create json file.");

        // Write json file
        // serde_json::to_writer(file, &gpu_data)
        //     .expect("Could not write data to json file");



        // Store sub-window states in settings
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
