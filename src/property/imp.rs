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
 * 05/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */

// Imports
use glib::{once_cell::sync::Lazy, ParamSpec, ToValue, Value};
use gtk::{glib, subclass::prelude::*};
use std::cell::Cell;

// Modules
use crate::formatter::Formatter;
use crate::processor::Processor;

// Object holding the state
#[derive(Default)]
pub struct Property {
    processor: Cell<Processor>,
    call_extension: Cell<String>,
    icon: Cell<String>,
    formatter: Cell<Formatter>,
    gpu_count: Cell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Property {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaExtensionRustProperty";
    // the actual GObject that will be created
    type Type = super::Property;
    // Parent GObject we inherit from
    type ParentType = gtk::Widget;
}

/*
 * Trait Name:
 * ObjectImpl
 *
 * Description:
 * Trait shared by all GObjects
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
impl ObjectImpl for Property {
    /*
     * Name:
     * properties
     *
     * Description:
     * Create list of custom properties for our GObject
     *
     * Made:
     * 05/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * beware that you need to use kebab-case (https://en.wikipedia.org/wiki/Letter_case#Kebab_case)
     */
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::builder("icon").build(),
                glib::ParamSpecInt::builder("gpu-count").build(),
                glib::ParamSpecString::builder("call-extension").build(),
                glib::ParamSpecObject::builder("processor", glib::Type::OBJECT).build(),
                glib::ParamSpecObject::builder("formatter", glib::Type::OBJECT).build(),
            ]
        });

        //println!("PROPERTIES: {:?}", PROPERTIES);//TEST
        //println!("trying to add `base_call`: {:?}", glib::ParamSpecString::builder("base_call").build());//TEST

        PROPERTIES.as_ref()
    }

    /*
     * Name:
     * set_property
     *
     * Description:
     * Mutator for custom GObject properties
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
    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        //println!("setting: {:?}", pspec.name());//TEST

        match pspec.name() {
            "icon" => {
                let input_icon = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.icon.replace(input_icon);
            }
            "gpu-count" => {
                let input_gpu_count = value.get().expect("The value needs to be of type `i32`.");
                self.gpu_count.replace(input_gpu_count);
            }
            "call-extension" => {
                let input_call_extension = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.call_extension.replace(input_call_extension);
            }
            "processor" => {
                let input_processor = value
                    .get()
                    .expect("The value needs to be of type `Processor`.");
                self.processor.replace(input_processor);
            }
            "formatter" => {
                let input_formatter = value
                    .get()
                    .expect("The value needs to be of type `Formatter`.");
                self.formatter.replace(input_formatter);
            }
            _ => unimplemented!(), //TODO
        }
    }

    /*
     * Name:
     * property
     *
     * Description:
     * Accessir for custom GObject properties
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
    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        //println!("getting: {:?}", pspec.name());//TEST

        match pspec.name() {
            "icon" => {
                //TODO: this seems ridiculous..
                let value = self.icon.take();

                self.icon.set(value.clone());

                value.to_value()
            }
            "gpu-count" => {
                //TODO: this seems ridiculous..
                let value = self.gpu_count.take();

                self.gpu_count.set(value);

                value.to_value()
            }
            "call-extension" => {
                //TODO: this seems ridiculous..
                let value = self.call_extension.take();

                self.call_extension.set(value.clone());

                value.to_value()
            }
            "processor" => {
                //TODO: this seems ridiculous..
                let value = self.processor.take();

                self.processor.set(value.clone());

                value.to_value()
            }
            "formatter" => {
                //TODO: this seems ridiculous..
                let value = self.formatter.take();

                self.formatter.set(value.clone());

                value.to_value()
            }
            _ => unimplemented!(), //TODO
        }
    }
}

/*
 * Trait Name:
 * WidgetImpl
 *
 * Description:
 * Trait shared by all widgets
 *
 * Made:
 * 05/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * leaving blank atm, boilerplate
 */
impl WidgetImpl for Property {}
