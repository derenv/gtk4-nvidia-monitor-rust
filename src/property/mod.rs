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
 * 05/10/2022
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
use gtk::prelude::ObjectExt;

// Modules
use crate::processor::Processor;
use crate::formatter::{Formatter};

// GObject wrapper for Property
glib::wrapper! {
    pub struct Property(ObjectSubclass<imp::Property>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

/*
 * Trait Name:
 * Property
 *
 * Description:
 * Trait shared by all properties
 *
 * Made:
 * 05/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl Property {
    /*
     * Name:
     * new
     *
     * Description:
     * Create a new Property object
     *
     * Made:
     * 05/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * processor, name, callExtension, icon, formatter, gpuCount
     *     ^      skip  utilization.gpu  ??     ^         i32
     *
     * given proc and gpuCount
     */
    pub fn new(
        processor: &Processor,
        call_extension: &str,
        icon: &str,
        formatter: &Formatter,
        gpu_count: &i32
    ) -> Self {
        let obj: Property = Object::new(&[]).expect("Failed to create `Property`.");

        // TODO: set properties
        obj.set_property("processor", processor);
        obj.set_property("call-extension", call_extension.to_string());
        obj.set_property("icon", icon.to_string());
        obj.set_property("formatter", formatter);
        obj.set_property("gpu_count", gpu_count);

        obj
    }

    // Parsing
    //https://doc.rust-lang.org/std/primitive.array.html
    //https://www.tutorialspoint.com/rust/rust_array.htm
    //https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn parse(self, values: Vec<Vec<String>>, func: fn(Vec<String>) -> Option<String>) -> Option<Vec<String>> {
        let mut results: Vec<String> = Vec::new();

        // For each GPU
        for i in 0..self.property::<i32>("gpu-count") {
            // format properties using formatter and add to return values
            let formatter = self.property::<Formatter>("formatter");
            match formatter.format(values.get(i as usize).unwrap().clone(), func) {
                Some(result) => results.push(result),
                None => return None,
            }
        }

        Some(results)
    }

    pub fn get_call_extension(&self) -> String {
        self.property::<String>("call-extension").to_owned()
    }
}
