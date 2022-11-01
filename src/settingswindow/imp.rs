// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (SettingsWindow)
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

// Imports
use adwaita::{gio, glib, prelude::*, subclass::prelude::*, ComboRow};
use gio::Settings;
use glib::{once_cell::sync::OnceCell, signal::Inhibit, subclass::InitializingObject};
use gtk::{subclass::prelude::*, CheckButton, CompositeTemplate, SpinButton, TemplateChild};

// Modules
//use crate::utils::data_path;

/// Object holding the State and any Template Children
#[derive(CompositeTemplate, Default)]
#[template(resource = "/settings-window.ui")]
pub struct SettingsWindow {
    pub settings: OnceCell<Settings>,
    #[template_child]
    pub refreshrate_input: TemplateChild<SpinButton>,
    #[template_child]
    pub temp_unit_c: TemplateChild<CheckButton>,
    #[template_child]
    pub temp_unit_f: TemplateChild<CheckButton>,
    #[template_child]
    pub provider_input: TemplateChild<ComboRow>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SettingsWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NvidiaExtensionSettingsWindow";
    type Type = super::SettingsWindow;
    //type ParentType = adwaita::PreferencesWindow;
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
 * SettingsWindow
 *
 * Description:
 * Trait shared by all SettingsWindow objects
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
impl SettingsWindow {
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

    #[template_callback]
    fn refreshrate_set(&self, button: &SpinButton) {
        // Get new refresh rate input
        let new_value: i32 = button.value_as_int();

        // Set refresh rate property
        self.update_setting("refreshrate", new_value);
    }

    #[template_callback]
    fn temp_unit_set(&self, button: &CheckButton) {
        // Get list of buttons
        let check_buttons: [&CheckButton; 2] = [&self.temp_unit_c, &self.temp_unit_f];

        // For each button in the group
        for current_button in check_buttons {
            // Check if current button active
            if current_button.is_active() {
                // Get new unit
                match button.label() {
                    Some(unit) => {
                        // Set appropriate setting
                        match String::from(unit).as_str() {
                            "Celcius (C)" => {
                                // Set temperature unit as C
                                self.update_setting("tempformat", 0);
                            }
                            "Fahrenheit (F)" => {
                                // Set temperature unit as F
                                self.update_setting("tempformat", 1);
                            }
                            _ => panic!("..Unexpected temperature unit")
                        }
                    },
                    None => panic!("..Could not fetch contents of temperature unit button label"),
                }
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
 * 10/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ObjectImpl for SettingsWindow {
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
        obj.restore_data();
        obj.setup_widgets();
        obj.setup_callbacks();
        obj.setup_actions();
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
 * 10/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl WidgetImpl for SettingsWindow {}

/**
 * Trait Name:
 * WindowImpl
 *
 * Description:
 * Trait shared by all Window's
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
impl WindowImpl for SettingsWindow {
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

        // Fetch settings
        match self.settings.get() {
            Some(settings) => {
                // Set state in settings
                match settings.set_boolean("app-settings-open", false) {
                    Ok(_) => println!("..Settings open/closed state updated!"),
                    Err(_) => panic!("..Cannot set `app-settings-open` setting"),
                }
            },
            None => panic!("..Cannot retrieve settings")
        }

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
 * 10/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl AdwWindowImpl for SettingsWindow {}

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
impl ApplicationWindowImpl for SettingsWindow {}

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
impl AdwApplicationWindowImpl for SettingsWindow {}
