// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
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
 * <https://github.com/gtk-rs/gtk4-rs/blob/master/book/src/g_object_properties.md>
 * <https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/4/custom_button/imp.rs>
 * <https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/4/custom_button/mod.rs>
 */

// Imports
use std::cell::Cell;
use glib::{once_cell::sync::Lazy, ParamSpec, Value};
use gtk::{prelude::*, subclass::prelude::*};
use adwaita::glib;

/// Object holding the State and any Template Children
#[derive(Default)]
pub struct Processor {
    base_call: Cell<String>,
    call: Cell<String>,
    tail_call: Cell<String>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Processor {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaMonitorRustProcessor";
    // the actual GObject that will be created
    type Type = super::Processor;
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
impl ObjectImpl for Processor {
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
     *
     * ParamSpec Examples:
     * glib::ParamSpecString::builder("icon").build(),
     * glib::ParamSpecUInt::builder("gpu_count").build(),
     * glib::ParamSpecString::builder("call_extension").build(),
     * TODO: these are from property class
     * glib::ParamSpecBoxed::builder("processor").build(),
     * glib::ParamSpecObject::builder("formatter").build(),
     */
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::builder("base-call").build(),
                glib::ParamSpecString::builder("call").build(),
                glib::ParamSpecString::builder("tail-call").build(),
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
            "base-call" => {
                match value.get() {
                    Ok(input_base_call) => {
                        self.base_call.replace(input_base_call);
                    },
                    Err(_) => panic!("The value needs to be of type `String`."),
                }
            }
            "call" => {
                match value.get() {
                    Ok(input_call) => {
                        self.call.replace(input_call);
                    },
                    Err(_) => panic!("The value needs to be of type `String`."),
                }
            }
            "tail-call" => {
                match value.get() {
                    Ok(input_tail_call) => {
                        self.tail_call.replace(input_tail_call);
                    },
                    Err(_) => panic!("The value needs to be of type `String`."),
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
            "base-call" => {
                //TODO: this seems ridiculous..
                let value: String = self.base_call.take();

                self.base_call.set(value.clone());

                value.to_value()
            }
            "call" => {
                //TODO: this seems ridiculous..
                let value: String = self.call.take();

                self.call.set(value.clone());

                value.to_value()
            }
            "tail-call" => {
                //TODO: this seems ridiculous..
                let value: String = self.tail_call.take();

                self.tail_call.set(value.clone());

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
 * 18/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * leaving blank atm, boilerplate
 */
impl WidgetImpl for Processor {}
