// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (Property)
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

// Imports
use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct Property {
    str : processor,
    str : name,
    str : callExtension,
    str : icon,
    str : formatter,
    str : gpuCount,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Property {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaExtensionRustProperty";
    // the actual GObject that will be created
    type Type = super::Property;
    // Parent GObject we inherit from
    type ParentType = gtk::Object;
}

// Trait shared by all GObjects
impl ObjectImpl for Property {}
