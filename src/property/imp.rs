// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
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

/// Object holding the State and any Template Children
#[derive(Default)]
pub struct Property {
    processor: Cell<Processor>,
    call_extension: Cell<String>,
    icon: Cell<String>,
    formatter: Cell<Formatter>,
    gpu_count: Cell<i32>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Property {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaMonitorRustProperty";
    // the actual GObject that will be created
    type Type = super::Property;
    // Parent GObject we inherit from
    type ParentType = gtk::Widget;
}

/**
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
    /**
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
     * beware that you need to use kebab-case (<https://en.wikipedia.org/wiki/Letter_case#Kebab_case>)
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

    /**
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
                match value.get() {
                    Ok(input_icon) => {
                        self.icon.replace(input_icon);
                    },
                    Err(_) => panic!("The value needs to be of type `String`."),
                }
            }
            "gpu-count" => {
                match value.get() {
                    Ok(input_gpu_count) => {
                        self.gpu_count.replace(input_gpu_count);
                    },
                    Err(_) => panic!("The value needs to be of type `i32`."),
                }
            }
            "call-extension" => {
                match value.get() {
                    Ok(input_call_extension) => {
                        self.call_extension.replace(input_call_extension);
                    },
                    Err(_) => panic!("The value needs to be of type `String`."),
                }
            }
            "processor" => {
                match value.get() {
                    Ok(input_processor) => {
                        self.processor.replace(input_processor);
                    },
                    Err(_) => panic!("The value needs to be of type `Processor`."),
                }
            }
            "formatter" => {
                match value.get() {
                    Ok(input_formatter) => {
                        self.formatter.replace(input_formatter);
                    },
                    Err(_) => panic!("The value needs to be of type `Formatter`."),
                }
            }
            _ => panic!("Property `{}` does not exist..", pspec.name())
        }
    }

    /**
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
            _ => panic!("Property `{}` does not exist..", pspec.name())
        }
    }
}

/**
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
