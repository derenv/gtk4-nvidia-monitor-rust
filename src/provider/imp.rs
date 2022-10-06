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
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::{self, ParamSpec, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

// Object holding the State
#[derive(Default)]
pub struct Provider;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Provider {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaExtensionRustProcessor";
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
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| vec![]);

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
