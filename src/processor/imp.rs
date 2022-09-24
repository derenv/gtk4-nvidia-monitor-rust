// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (Processor)
 *
 * Made:
 * 18/09/2022
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
pub struct Processor {
    name: String,
    base_call: String,
    tail_call: String,
    call: String,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Processor {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaExtensionRustProcessor";
    // the actual GObject that will be created
    type Type = super::Processor;
    // Parent GObject we inherit from
    type ParentType = gtk::Widget;
}

// Trait shared by all GObjects
impl ObjectImpl for Processor {}

// Trait shared by all widgets
impl WidgetImpl for Processor {}
