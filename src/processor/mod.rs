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
use glib::Object;
use gtk::glib::{self, Bytes};
use super::subprocess;
use std::ffi::OsStr;

glib::wrapper! {
    //pub struct Processor(ObjectSubclass<imp::Processor<'a>>)
    pub struct Processor(ObjectSubclass<imp::Processor>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

// Trait shared by all processors
impl Processor {
    fn new(name: &'static str, base_call: &'static str, call: &'static str, tail_call: &'static str) -> Self {
        Object::new(&[
            ("name", &name),
            ("base_call", &base_call),
            ("call", &base_call),
            ("tail_call", &tail_call),
        ]).expect("Failed to create `Processor`.")
    }

    fn process(self) -> Vec<String> {
        // Create string of args
        /*
        let call: String = String::from("nvidia-settings");
        let tail_call: &str = "-t";
        call.push_str(tail_call);
        let call_b = call.as_bytes();

        let start: usize = 0;

        for (i, &item) in call_b.iter().enumerate() {
            if
        }
        */
        /*
        match subprocess::exec_communicate(self.argv, None, None) {
            Ok(output) => {
                let parse_output = self.parse(output);

                return parse_output;
            },
            Err(err) => todo!(),
        };
        */
        todo!()
    }

    fn add_property(self, call_extension: &str) {
        todo!()
        //self.call.push(call_extension);
    }

    fn get_name(self) -> () {//&str {
        todo!()
        //self.name.
    }

    fn parse(self, input: (Option<Bytes>, Option<Bytes>)) -> Vec<String> {
        todo!()
    }
}
