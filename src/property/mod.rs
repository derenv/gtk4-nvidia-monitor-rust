// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
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

use gdk::glib::value::FromValue;
// Imports
use glib::Object;
use gtk::{glib, prelude::*};

// Modules
use crate::formatter::Formatter;
use crate::processor::Processor;

// GObject wrapper for Property
glib::wrapper! {
    pub struct Property(ObjectSubclass<imp::Property>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

/**
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
    /**
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
        gpu_count: &i32,
    ) -> Self {
        let obj: Property = Object::new(&[]).expect("Failed to create `Property`.");

        // Set properties
        obj.set_property("processor", processor);
        obj.set_property("call-extension", String::from(call_extension));
        obj.set_property("icon", String::from(icon));
        obj.set_property("formatter", formatter);
        obj.set_property("gpu-count", gpu_count);

        obj
    }

    /**
     * Name:
     * parse
     *
     * Description:
     *
     *
     * Made:
     * 06/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * <https://doc.rust-lang.org/std/primitive.array.html>
     * <https://www.tutorialspoint.com/rust/rust_array.htm>
     * <https://doc.rust-lang.org/std/vec/struct.Vec.html>
     */
    pub fn parse(
        self,
        values: Vec<Vec<String>>,
        func: fn(Vec<String>) -> Option<String>,
    ) -> Option<Vec<String>> {
        // Initialise results vector
        let mut results: Vec<String> = Vec::new();

        // For each GPU
        for i in 0..self.property::<i32>("gpu-count") {
            // Grab formatter
            let formatter: Formatter = self.property::<Formatter>("formatter");

            // Check item exists
            match values.get(i as usize) {
                Some(valid_data) => {
                    // Format properties using formatter and add to return values
                    match formatter.format(valid_data.to_owned(), func) {
                        Some(result) => results.push(result),
                        None => return None,//TODO: investigate
                    }
                },
                None => panic!("..Invalid data values"),
            }
        }

        Some(results)
    }

    /**
     * Name:
     * get_value
     *
     * Description:
     * Get a property and return it's value
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
    pub fn get_value<T: for<'b> FromValue<'b> + 'static>(&self, name: &str) -> T {
        // Return the value of the property
        self.property::<T>(name)
    }

    /**
     * Name:
     * update_value
     *
     * Description:
     * Update a property with a new value
     *
     * Made:
     * 29/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn update_value<T: ToValue>(&self, property_name: &str, value: T) {
        // Update the property with new value
        self.set_property(property_name, value);
    }
}

/**
 * Trait Name:
 * Default
 *
 * Description:
 * Default object
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
impl Default for Property {
    fn default() -> Self {
        Self::new(&Processor::new("", ""), &"", &"", &Formatter::new(), &0)
    }
}
