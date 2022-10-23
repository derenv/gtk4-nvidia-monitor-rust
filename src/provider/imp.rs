// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (Provider)
 *
 * Made:
 * 06/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */

// Imports
use glib::{once_cell::sync::Lazy, ParamSpec, Value};
use gtk::{glib, prelude::*, subclass::prelude::*};
use std::cell::Cell;

// Modules
use crate::property::Property;

// Object holding the State
#[derive(Default)]
pub struct Provider {
    utilization: Cell<Property>,
    temperature: Cell<Property>,
    memory_usage: Cell<Property>,
    fan_speed: Cell<Property>,
    power_usage: Cell<Property>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Provider {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaMonitorRustProcessor";
    // the actual GObject that will be created
    type Type = super::Provider;
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
 * 06/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ObjectImpl for Provider {
    /*
     * Name:
     * properties
     *
     * Description:
     * Create list of custom properties for our GObject
     *
     * Made:
     * 06/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * beware that you need to use kebab-case (https://en.wikipedia.org/wiki/Letter_case#Kebab_case)
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
                glib::ParamSpecObject::builder("utilization-property", glib::Type::OBJECT).build(),
                glib::ParamSpecObject::builder("temperature-property", glib::Type::OBJECT).build(),
                glib::ParamSpecObject::builder("memory-usage-property", glib::Type::OBJECT).build(),
                glib::ParamSpecObject::builder("fan-speed-property", glib::Type::OBJECT).build(),
                glib::ParamSpecObject::builder("power-usage-property", glib::Type::OBJECT).build(),
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
     * 06/10/2022
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
            "utilization" => {
                let input_utilization_property = value
                    .get()
                    .expect("The value needs to be of type `Property`.");
                self.utilization.replace(input_utilization_property);
            }
            "temperature" => {
                let input_temperature_property = value
                    .get()
                    .expect("The value needs to be of type `Property`.");
                self.temperature.replace(input_temperature_property);
            }
            "memory-usage" => {
                let input_memory_usage_property = value
                    .get()
                    .expect("The value needs to be of type `Property`.");
                self.memory_usage.replace(input_memory_usage_property);
            }
            "fan-speed" => {
                let input_fan_speed_property = value
                    .get()
                    .expect("The value needs to be of type `Property`.");
                self.fan_speed.replace(input_fan_speed_property);
            }
            "power-usage" => {
                let input_power_usage_property = value
                    .get()
                    .expect("The value needs to be of type `Property`.");
                self.power_usage.replace(input_power_usage_property);
            }
            _ => unimplemented!(), //TODO
        }
    }

    /*
     * Name:
     * property
     *
     * Description:
     * Accessor for custom GObject properties
     *
     * Made:
     * 06/10/2022
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
            "utilization" => {
                //TODO: this seems ridiculous..
                let value = self.utilization.take();

                self.utilization.set(value.clone());

                value.to_value()
            }
            "temperature" => {
                //TODO: this seems ridiculous..
                let value = self.temperature.take();

                self.temperature.set(value.clone());

                value.to_value()
            }
            "memory-usage" => {
                //TODO: this seems ridiculous..
                let value = self.memory_usage.take();

                self.memory_usage.set(value.clone());

                value.to_value()
            }
            "fan-speed" => {
                //TODO: this seems ridiculous..
                let value = self.fan_speed.take();

                self.fan_speed.set(value.clone());

                value.to_value()
            }
            "power-usage" => {
                //TODO: this seems ridiculous..
                let value = self.power_usage.take();

                self.power_usage.set(value.clone());

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
 * 06/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * leaving blank atm, boilerplate
 */
impl WidgetImpl for Provider {}
