// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (ModificationWindow)
 *
 * Made:
 * 04/12/2022
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
use adwaita::{
    gio,
    glib,
    subclass::prelude::*,
    prelude::*,
};
use gio::Settings;
use glib::{Object, GString};

// Modules
use crate::{APP_ID, modificationwindow::imp::ViewComponent};

// GObject wrapper for ModificationWindow
glib::wrapper! {
    pub struct ModificationWindow(ObjectSubclass<imp::ModificationWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

/**
 * Trait Name:
 * ModificationWindow
 *
 * Description:
 * Trait shared by all main windows
 *
 * Made:
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ModificationWindow {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new ModificationWindow object
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn new(app: &adwaita::Application, view_id: i32, uuid: &str) -> Self {
        let obj: ModificationWindow = Object::new(&[("application", app)]).expect("`ModificationWindow` should be  instantiable.");

        // Set custom properties
        obj.set_property("old-view-id", view_id);
        obj.set_property("new-view-id", view_id);
        obj.set_property("uuid", String::from(uuid));

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
     * 04/12/2022
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
            .expect("`settings` should not be set before calling `setup_settings`..");
    }

    /**
     * Name:
     * settings
     *
     * Description:
     * Get settings for APP_ID
     *
     * Made:
     * 04/12/2022
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
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn setup_widgets(&self) {
        // Retrieve names of stored views
        let view_title_list: Vec<GString> = self.settings().strv("viewconfigs");
        let view_id: String = self.property::<i32>("old-view-id").to_string();

        // Create empty string for the title
        let mut view_title: String = String::from("Default");

        // Retrieve name of current view
        for item in view_title_list {
            let sub_items: Vec<&str> = item.split(':').collect();
            if sub_items[1] == view_id {
                view_title = sub_items[2].to_owned();
                break;
            }
        }
        println!("   View ID: {}", view_id); //TEST
        println!("View Title: {}", view_title); //TEST

        // Store name of current view
        self.set_property("old-view-title", view_title.clone());
        self.set_property("new-view-title", view_title);



        // Retrieve list of in-use properties
        let view_components_list = self.settings().strv("viewcomponentconfigs");
        println!("Possible Components: {:?}", view_components_list); //TEST

        // Create list of components in current view
        let mut final_components: Vec<ViewComponent> = vec![];

        for item in view_components_list {
            println!("item: `{}`", item); //TEST
            let sub_items: Vec<&str> = item.split(':').collect();
            if sub_items[1] == self.property::<String>("old-view-title") {
                println!("View Component: {}", sub_items[3]); //TEST

                // Create new item
                let new_item: ViewComponent = ViewComponent {
                    name: String::from(sub_items[3]),
                    position: sub_items[2].parse::<i32>().expect("Malformed gschema data..")
                };
                final_components.push(new_item);

                // Change state of appropriate checkbutton
                match sub_items[3] {
                    "util" => {
                        println!("`util` active"); //TEST
                        self.imp().util_checkbox.set_active(true);
                    },
                    "temp" => {
                        println!("`temp` active"); //TEST
                        self.imp().temp_checkbox.set_active(true);
                    },
                    "power_usage" => {
                        println!("`power_usage` active"); //TEST
                        self.imp().power_usage_checkbox.set_active(true);
                    },
                    "memory_usage" => {
                        println!("`memory_usage` active"); //TEST
                        self.imp().mem_usage_checkbox.set_active(true);
                    },
                    "memory_total" => {
                        println!("`memory_total` active"); //TEST
                        self.imp().mem_total_checkbox.set_active(true);
                    },
                    "mem_ctrl_util" => {
                        println!("`mem_ctrl_util` active"); //TEST
                        self.imp().mem_util_checkbox.set_active(true);
                    },
                    "fan_speed" => {
                        println!("`fan_speed` active"); //TEST
                        self.imp().fan_speed_checkbox.set_active(true);
                    },
                    _ => panic!("unknown property..."),
                }
            }
        }

        // Bind components list to struct member
        self.imp().view_components_list.set(final_components);

    }

    /**
     * Name:
     * restore_data
     *
     * Description:
     * Restore properties from settings
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * TODO
     */
    fn restore_data(&self) {
        //
    }

    /**
     * Name:
     * setup_callbacks
     *
     * Description:
     * Child widget callbacks
     *
     * Made:
     * 04/12/2022
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
     * 04/12/2022
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
