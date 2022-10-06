// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (Provider)
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

// Custom GObjects
mod imp;

// Imports
use super::subprocess;
use glib::Object;
use gtk::{gio, glib, prelude::ObjectExt};
use std::ffi::OsStr;

// GObject wrapper for Provider
glib::wrapper! {
    pub struct Provider(ObjectSubclass<imp::Provider>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

/*
 * Trait Name:
 * Provider
 *
 * Description:
 * Trait shared by all Providers
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
impl Provider {
    /*
     * Name:
     * new
     *
     * Description:
     * Create a new Provider object
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
    pub fn new() -> Self {
        let obj: Provider = Object::new(&[]).expect("Failed to create `Provider`");

        // TODO: set properties
        //obj.set_property("call", base_call.to_string().clone());

        obj
    }
}

/*
 * Trait Name:
 * Default
 *
 * Description:
 * Default object
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
impl Default for Provider {
    fn default() -> Self {
        Self::new()
    }
}
