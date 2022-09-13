// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (Property)
 *
 * Made:
 * 13/09/2022
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
use gtk::glib;

glib::wrapper! {
    pub struct Property(ObjectSubclass<imp::Property>)
        @extends glib::Object;/*,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;*/
}

impl Property {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `Property`.")
    }
    /*
    pub fn with_label(label: &str) -> Self {
        Object::new(&[("label", &label)]).expect("Failed to create `Property`.")
    }
    */
}
