// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
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
 * It'll be easier to pass a defined "parse_function" to new objects rather than define 3 new classes
 * However - getting that working with generics and lifetimes is a bitch..
 */

// Custom GObjects
mod imp;

// Imports
use super::subprocess;
use glib::Object;
use gtk::{gio, glib, prelude::ObjectExt};
use std::ffi::OsStr;

// GObject wrapper for Processor
glib::wrapper! {
    pub struct Processor(ObjectSubclass<imp::Processor>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

/**
 * Trait Name:
 * Processor
 *
 * Description:
 * Trait shared by all processors
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
impl Processor {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new Processor object
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
    pub fn new(base_call: &str, tail_call: &str) -> Self {
        let obj: Processor = Object::new(&[]).expect("Failed to create `Processor`");

        // Set properties
        obj.set_property("base-call", String::from(base_call));
        obj.set_property("call", String::from(base_call).clone());
        obj.set_property("tail-call", String::from(tail_call));

        obj
    }

    /**
     * Name:
     * process
     *
     * Description:
     * Runs call stack and return result
     *
     * Made:
     * 18/09/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * we'll know what possible sizes will exist (wherever this gets implemented)
     */
    pub fn process(self) -> Result<Option<Vec<String>>, glib::Error> {
        // Create call stack of program and args
        let tail_call = self.property::<String>("tail-call");
        let mut call_stack = self.property::<String>("call");
        call_stack.push(' ');
        call_stack.push_str(tail_call.as_str());

        // Turn call stack into bytes and create vector for final call stack
        let call_stack_bytes: &[u8] = call_stack.as_bytes();
        let mut call_stack_items: Vec<&OsStr> = Vec::new();

        // Build OsStr type vector of all args
        let mut start: usize = 0;
        for (i, &item) in call_stack_bytes.iter().enumerate() {
            // if space
            if item == b' ' {
                let item_osstr: &OsStr;
                match std::str::from_utf8(&call_stack_bytes[start..i]) {
                    Ok(result) => {
                        //println!("item: {}", result);//TEST
                        item_osstr = OsStr::new(result)
                    }
                    Err(err) => panic!("{}", err),
                }
                call_stack_items.insert(call_stack_items.len(), item_osstr);
                start = i + 1;
            }
            // if final char
            if i == (call_stack_bytes.iter().len() - 1) {
                let item_osstr: &OsStr;
                match std::str::from_utf8(&call_stack_bytes[start..]) {
                    Ok(result) => {
                        //println!("item: {}", result);//TEST
                        item_osstr = OsStr::new(result)
                    }
                    Err(err) => panic!("{}", err),
                }
                call_stack_items.insert(call_stack_items.len(), item_osstr);
            }
        }

        // Build OsStr array from vector (if matching a specific size)
        match call_stack_items.len() {
            6 => {
                //optirun nvidia-smi --query-gpu=gpu_name --format=csv,noheader -i uuid
                // Build array
                let argv: [&OsStr; 6] = [
                    call_stack_items[0],
                    call_stack_items[1],
                    call_stack_items[2],
                    call_stack_items[3],
                    call_stack_items[4],
                    call_stack_items[5],
                ];

                // Run process, get output
                match subprocess::exec_communicate(&argv, None::<&gio::Cancellable>) {
                    Ok(return_val) => match return_val {
                        // ACTUAL
                        (None, None) => return Ok(None),

                        (None, Some(stderr_buffer)) => {
                            println!(
                                "Process failed with error: {}",
                                String::from_utf8_lossy(&stderr_buffer)
                            );
                        }

                        (Some(stdout_buffer), None) => return Ok(Some(self.parse(&String::from_utf8_lossy(&stdout_buffer)))),

                        (Some(stdout_buffer), Some(stderr_buffer)) => {
                            println!(
                                "Process succeeded, but with error: {}",
                                String::from_utf8_lossy(&stderr_buffer)
                            );

                            return Ok(Some(self.parse(&String::from_utf8_lossy(&stdout_buffer))));
                        }
                    },
                    Err(err) => return Err(err),
                };
            }
            5 => {
                //nvidia-smi --query-gpu=gpu_name --format=csv,noheader -i uuid
                // Build array
                let argv: [&OsStr; 5] = [
                    call_stack_items[0],
                    call_stack_items[1],
                    call_stack_items[2],
                    call_stack_items[3],
                    call_stack_items[4],
                ];

                // Run process, get output
                match subprocess::exec_communicate(&argv, None::<&gio::Cancellable>) {
                    Ok(return_val) => match return_val {
                        // ACTUAL
                        (None, None) => return Ok(None),

                        (None, Some(stderr_buffer)) => {
                            println!(
                                "Process failed with error: {}",
                                String::from_utf8_lossy(&stderr_buffer)
                            );
                        }

                        (Some(stdout_buffer), None) => return Ok(Some(self.parse(&String::from_utf8_lossy(&stdout_buffer)))),

                        (Some(stdout_buffer), Some(stderr_buffer)) => {
                            println!(
                                "Process failed with error: {}",
                                String::from_utf8_lossy(&stderr_buffer)
                            );

                            return Ok(Some(self.parse(&String::from_utf8_lossy(&stdout_buffer))));
                        }
                    },
                    Err(err) => return Err(err),
                };
            }
            4 => {
                //nvidia-settings -q GpuUUID -t
                // Build array
                let argv: [&OsStr; 4] = [
                    call_stack_items[0],
                    call_stack_items[1],
                    call_stack_items[2],
                    call_stack_items[3],
                ];

                // Run process, get output
                match subprocess::exec_communicate(&argv, None::<&gio::Cancellable>) {
                    Ok(return_val) => match return_val {
                        // ACTUAL
                        (None, None) => return Ok(None),

                        (None, Some(stderr_buffer)) => {
                            println!(
                                "Process failed with error: {}",
                                String::from_utf8_lossy(&stderr_buffer)
                            );
                        }

                        (Some(stdout_buffer), None) => return Ok(Some(self.parse(&String::from_utf8_lossy(&stdout_buffer)))),

                        (Some(stdout_buffer), Some(stderr_buffer)) => {
                            println!(
                                "Process failed with error: {}",
                                String::from_utf8_lossy(&stderr_buffer)
                            );

                            return Ok(Some(self.parse(&String::from_utf8_lossy(&stdout_buffer))));
                        }
                    },
                    Err(err) => return Err(err),
                };
            }
            2 => {
                //??
                // Build array
                let argv: [&OsStr; 2] = [call_stack_items[0], call_stack_items[1]];

                // Run process, get output
                match subprocess::exec_communicate(&argv, None::<&gio::Cancellable>) {
                    Ok(return_val) => match return_val {
                        // ACTUAL
                        (None, None) => return Ok(None),

                        (None, Some(stderr_buffer)) => {
                            println!(
                                "Process failed with error: {}",
                                String::from_utf8_lossy(&stderr_buffer)
                            );
                        }

                        (Some(stdout_buffer), None) => return Ok(Some(self.parse(&String::from_utf8_lossy(&stdout_buffer)))),

                        (Some(stdout_buffer), Some(stderr_buffer)) => {
                            println!(
                                "Process failed with error: {}",
                                String::from_utf8_lossy(&stderr_buffer)
                            );

                            return Ok(Some(self.parse(&String::from_utf8_lossy(&stdout_buffer))));
                        }
                    },
                    Err(err) => return Err(err),
                };
            }
            _invalid_size => return Ok(None), // This will only occur via programmer error
        }

        Ok(None)
    }

    /**
     * Name:
     * parse
     *
     * Description:
     * Grab input string as owned, append test formatting and then return
     *
     * Made:
     * 18/09/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * This function is designed to be overloaded by subclasses
     */
    fn parse(self, input: &str) -> Vec<String> {
        //NOTE: leaving this here for future use..
        //let mut output = input.replace("\n", "").to_owned();
        //output.push_str("-FUCK");

        let mut return_val: Vec<String> = vec![];

        for item in input.lines().collect::<Vec<&str>>() {
            return_val.push(String::from(item));
            //return_val.insert(0, String::from(item));
        }

        return_val
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
 * 08/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl Default for Processor {
    fn default() -> Self {
        Self::new("", "")
    }
}
