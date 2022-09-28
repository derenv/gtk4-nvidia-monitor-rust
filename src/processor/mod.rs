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
 * It'll be easier to pass a defined "parse_function" to new objects rather than define 3 new classes
 * However - getting that working with generics and lifetimes is a bitch..
 */

// Custom GObjects
mod imp;

// Imports
use super::subprocess;
use glib::Object;
use gtk::{ glib, gio };
use std::ffi::OsStr;

glib::wrapper! {
    //pub struct Processor(ObjectSubclass<imp::Processor<'a>>)
    pub struct Processor(ObjectSubclass<imp::Processor>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

// Trait shared by all processors
impl Processor {
    pub fn new(name: &'static str, /*base_call: &'static str, tail_call: &'static str*/) -> Self {
        Object::new(&[
            ("name", &name),/*
            ("base_call", &base_call),
            ("call", &base_call.clone()),
            ("tail_call", &tail_call),*/
        ]).expect("Failed to create `Processor`")
    }

    pub fn process(self) -> Result<Option<String>, glib::Error> {
        // Create string of args
        //NOTE: we'll know what possible sizes will exist (wherever this gets implemented)
        let mut call_stack: String = String::from("nvidia-settings");
        let tail_call: &str = "-q GpuUUID -t";
        call_stack.push_str(" ");
        call_stack.push_str(tail_call);
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
                        println!("item: {}", result);
                        item_osstr = OsStr::new(result)
                    },
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
                        println!("item: {}", result);
                        item_osstr = OsStr::new(result)
                    },
                    Err(err) => panic!("{}", err),
                }
                call_stack_items.insert(call_stack_items.len(), item_osstr);
            }
        }

        // Build OsStr array from vector
        match call_stack_items.len() {
            4 => {
                // Build array
                let argv = [call_stack_items[0], call_stack_items[1], call_stack_items[2], call_stack_items[3]];

                // Run process, get output
                match subprocess::exec_communicate(&argv, None::<&gio::Cancellable>) {
                    Ok(return_val) => match return_val {
                        // ACTUAL
                        (None, None) => return Ok(None),

                        (None, Some(stderr_buffer)) => {
                            println!("Process failed with error: {}", String::from_utf8_lossy(&stderr_buffer).into_owned());
                        },

                        (Some(stdout_buffer), None) => {
                            let stdout_buffer_contents = String::from_utf8_lossy(&stdout_buffer).into_owned();

                            return Ok(Some(self.parse(&stdout_buffer_contents)));
                        },

                        (Some(stdout_buffer), Some(stderr_buffer)) => {
                            let stdout_buffer_contents = String::from_utf8_lossy(&stdout_buffer).into_owned();

                            println!("Process failed with error: {}", String::from_utf8_lossy(&stderr_buffer).into_owned());

                            return Ok(Some(self.parse(&stdout_buffer_contents)));
                        },
                    },
                    Err(err) => return Err(err),
                };
            },
            2 => {
                // Build array
                let argv = [call_stack_items[0], call_stack_items[1]];

                // Run process, get output
                match subprocess::exec_communicate(&argv, None::<&gio::Cancellable>) {
                    Ok(return_val) => match return_val {
                        // ACTUAL
                        (None, None) => return Ok(None),

                        (None, Some(stderr_buffer)) => {
                            println!("Process failed with error: {}", String::from_utf8_lossy(&stderr_buffer).into_owned());
                        },

                        (Some(stdout_buffer), None) => {
                            let stdout_buffer_contents = String::from_utf8_lossy(&stdout_buffer).into_owned();

                            return Ok(Some(self.parse(&stdout_buffer_contents)));
                        },

                        (Some(stdout_buffer), Some(stderr_buffer)) => {
                            let stdout_buffer_contents = String::from_utf8_lossy(&stdout_buffer).into_owned();

                            println!("Process failed with error: {}", String::from_utf8_lossy(&stderr_buffer).into_owned());

                            return Ok(Some(self.parse(&stdout_buffer_contents)));
                        },
                    },
                    Err(err) => return Err(err),
                };
            },
            _invalid_size => return Ok(None), // This will only occur via programmer error
        }

        return Ok(None);
    }

    /*
    fn add_property(self, call_extension: &str) {
        todo!()
        //self.call.push(call_extension);
    }

    fn get_name(self) -> () {//&str {
        todo!()
        //self.name.
    }

    fn parse(self, input: &String) -> String {
        // Grab input string as owned, then return
        //(this function is designed to be overloaded by subclasses)
        input.replace("\n", "").to_owned()
    }
    */

    fn parse(self, input: &String) -> String {
        // Grab input string as owned, append test formatting and then return
        let mut output = input.replace("\n", "").to_owned();
        output.push_str("-FUCK");

        output
    }
}
