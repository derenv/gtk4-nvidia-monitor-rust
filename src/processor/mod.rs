// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (Processor)
 *
 * Made:
 * 12/09/2022
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
    pub struct Processor(ObjectSubclass<imp::Processor>)
        @extends gtk::Widget,
        //@extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Processor {
    pub fn new(name: &str, base_call: &str, tail_call: &str) -> Self {
        Object::new(&[
            ("name", &name),
            ("base_call", &base_call),
            ("tail_call", &tail_call)
        ]).expect("Failed to create `Processor`.")
    }

    pub fn parse() {
        todo!()
    }

    pub fn process(self) {
        todo!()
    }

    pub fn add_property(self) {
        todo!()
    }

    pub fn get_name(self) -> String {
        todo!()
        //self.name
    }
}
