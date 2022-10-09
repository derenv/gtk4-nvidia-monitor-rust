// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (Window)
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
use std::cell::RefCell;

use adwaita::subclass::prelude::*;
use gtk::subclass::prelude::*;

use gio::Settings;
use glib::signal::Inhibit;
use glib::subclass::InitializingObject;

use adwaita::prelude::*;
use gtk::{gio, glib, CompositeTemplate, Entry, ListBox, TemplateChild};
use gtk::glib::once_cell::sync::OnceCell;

//use crate::task_object::{TaskData, TaskObject};
//use crate::utils::data_path;

//use crate::utils::data_path;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_d/NvidiaExtensionRust/window.ui")]
pub struct Window {
    pub settings: OnceCell<Settings>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NvidiaExtensionWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        // Setup
        obj.setup_settings();
        //obj.setup_tasks();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_actions();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {
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
        // Pass close request on to the parent
        self.parent_close_request(window)
    }
}
impl AdwWindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}
