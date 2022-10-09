// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (Provider)
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

// Custom GObjects
mod imp;

use crate::property::Property;

// Imports
use glib::Object;
use gtk::{ glib, prelude::ObjectExt };
//use shell::*;
use gtk::gio;

// GObject wrapper for Provider
glib::wrapper! {
    pub struct Provider(ObjectSubclass<imp::Provider>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

#[derive(Default)]
pub enum ProviderType {
    #[default]
    NvidiaSettings = 0,
    NvidiaSmi = 1,
    NvidiaOptimus = 2,
}

/*
 * Trait Name:
 * Provider
 *
 * Description:
 * Trait shared by all Providers
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
impl Provider {
    /*
     * Name:
     * new
     *
     * Description:
     * Create a new Provider object
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
    pub fn new(func: fn() -> Vec<Property>/*, provider_type: ProviderType*/) -> Self {
        let obj: Provider = Object::new(&[]).expect("Failed to create `Provider`");

        //obj.set_property("provider-type", provider_type);

        // Set properties
        let properties: Vec<Property> = func();
        obj.set_property("utilization", properties[0].clone());
        obj.set_property("temperature", properties[1].clone());
        obj.set_property("memory-usage", properties[2].clone());
        obj.set_property("fan-speed", properties[3].clone());
        if properties.len() == 5 {
            obj.set_property("power-usage", properties[4].clone());
        }

        obj
    }

    pub fn open_settings() {
        //let defaultAppSystem = Shell.AppSystem.get_default();
        //let nvidiaSettingsApp = defaultAppSystem.lookup_app('nvidia-settings.desktop');
        //let def = shell::Edge::Top;
        let dd = gio::DesktopAppInfo::from_filename("nvidia-settings.desktop");
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
 * 06/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl Default for Provider {
    fn default() -> Self {
        Self::new(|| Vec::new()/*, ProviderType::NvidiaSettings*/)
    }
}
