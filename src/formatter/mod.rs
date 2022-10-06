// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (Formatter)
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

// Modules
//

// GObject wrapper for Formatter
glib::wrapper! {
    pub struct Formatter(ObjectSubclass<imp::Formatter>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

/*
 * Trait Name:
 * Formatter
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
impl Formatter {
    /*
     * Name:
     * new
     *
     * Description:
     * Create a new Formatter object
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
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `Formatter`.")
    }

    pub fn format(
        self,
        values: Vec<String>,
        func: fn(Vec<String>) -> Option<String>,
    ) -> Option<String> {
        let mut results: Vec<String> = Vec::new();

        // For each item in input list
        for i in values {
            // Remove all non-number characters
            let cleaned_value: String = i
                .chars()
                .filter(|c| {
                    // check if (base 10) digit
                    if c.is_digit(10) {
                        true
                    } else {
                        // check if full-stop
                        c.eq(&'.')
                    }
                })
                .collect();

            // Convert to float
            match cleaned_value.parse::<f64>() {
                Ok(parsed_value) => {
                    // Convert to string
                    results.push(parsed_value.to_string());
                }
                Err(err) => {
                    // Catch any errors..
                    println!("Not a valid number: {}", err);
                }
            }
        }

        // Check for empty results
        if !results.is_empty() {
            // Apply any valid formatting
            func(results)
        } else {
            None
        }
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
 * 08/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}
