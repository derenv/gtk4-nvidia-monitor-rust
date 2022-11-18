// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
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

// Imports
use adwaita::{gio, glib};
use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::ffi::OsStr;

// Crates
use crate::{processor::Processor, property::Property, subprocess::subprocess::exec_check};

// GObject wrapper for Provider
glib::wrapper! {
    pub struct Provider(ObjectSubclass<imp::Provider>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

/**
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
    /**
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
     * NvidiaSettingsandNvidiaSmi = 0,
     * NvidiaSettings = 0,
     * NvidiaSmi = 1,
     * NvidiaOptimus = 2,
     *
     */
    pub fn new(func: fn() -> Vec<Property>, provider_type: i32) -> Self {
        let obj: Provider = Object::new(&[]).expect("Failed to create `Provider`");

        // Set type of provider
        obj.set_property("provider-type", provider_type);

        // Set properties
        let res: Vec<Property> = func();
        //println!("PROPERTIES ON CREATION: `{}`", res.len());
        obj.imp().properties.replace(res);

        obj
    }

    /**
     * Name:
     * get_gpu_uuids
     *
     * Description:
     * Get list of all GPU uuid's
     *
     * Made:
     * 24/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
    //NOTE: Leaving this here for future use..
    //p.set_property("base-call", "nvidia-settings");
    //p.set_property("call", "nvidia-settings");
    //p.set_property("tail-call", "t");
     */
    pub fn get_gpu_uuids(&self) -> Result<Vec<String>, String> {
        // Check provider type
        let processor_args: [&str; 2];
        match self.property::<i32>("provider-type") {
            // Nvidia Settings/SMI
            0 => {
                processor_args = [
                    "nvidia-settings",
                    "-q GpuUUID -t"
                ];
            }
            // Nvidia Settings
            1 => {
                processor_args = [
                    "nvidia-settings",
                    "-q GpuUUID -t"
                ];
            }
            // Nvidia SMI
            2 => {
                processor_args = [
                    "nvidia-smi",
                    "--query-gpu=gpu_name --format=csv,noheader"
                ];
            }
            // Nvidia Optimus
            3 => {
                processor_args = [
                    "optirun",
                    "nvidia-smi --query-gpu=gpu_name --format=csv,noheader"
                ];
            }
            _ => {
                // Return error..
                return Err(String::from("Invalid provider, check preferences.."))
            }
        }

        // Create a processor object with appropriate args
        let processor: Processor = Processor::new(
            processor_args[0],
            processor_args[1],
            ""
        );

        // Validate output
        match processor.process(None, None) {
            Ok(output) => match output {
                Some(valid_output) => {
                    // If a valid output given, finally return to main window
                    Ok(valid_output)
                }
                None => {
                    // Return error..
                    Err(String::from("Process encountered an unknown error.."))
                }
            },
            Err(err) => {
                // Return error..
                return Err(String::from(err.message()));
            }
        }
    }

    /**
     * Name:
     * get_gpu_data
     *
     * Description:
     * Grab gpu data from provider program given a GPU uuid and property name
     *
     * Made:
     * 30/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * Designed to be expanded on later when more data needed..
     *
        let statistics_data: Vec<&str> = vec![
            "util",
            "temp",
            "memory_usage",
            "memory_total",
            "fan_speed",
            "power_usage",
        ];
     */
    pub fn get_gpu_data(&self, uuid: &str, property: &str) -> Result<String, String> {
        //println!("ASKED TO FETCH: `{}`", property);//TEST
        //println!("TYPE: `{}`", self.property::<i32>("provider_type"));//TEST

        // Translate to appropriate name
        let final_property: String;
        match self.property::<i32>("provider_type") {
            // Nvidia Settings/SMI
            0 => match property {
                "name" => final_property = String::from("gpu_name"),
                "util" => final_property = String::from("utilization.gpu"),
                "temp" => final_property = String::from("temperature.gpu"),
                "memory_usage" => final_property = String::from("memory.used"),
                "memory_total" => final_property = String::from("memory.total"),
                "fan_speed" => final_property = String::from("fan.speed"),
                "power_usage" => final_property = String::from("power.draw"),
                _ => return Err(String::from("Unknown property.."))
            },
            // Nvidia Settings
            1 => match property {
                "util" => final_property = String::from("GPUUtilization"),
                "temp" => final_property = String::from("GPUCoreTemp"),
                "memory_usage" => final_property = String::from("UsedDedicatedGPUMemory"),
                "memory_total" => final_property = String::from("TotalDedicatedGPUMemory"),
                "fan_speed" => final_property = String::from("GPUCurrentFanSpeedRPM"),
                _ => return Err(String::from("Unknown property.."))
            },
            // Nvidia SMI
            2 => match property {
                "name" => final_property = String::from("gpu_name"),
                "util" => final_property = String::from("utilization.gpu"),
                "temp" => final_property = String::from("temperature.gpu"),
                "memory_usage" => final_property = String::from("memory.used"),
                "memory_total" => final_property = String::from("memory.total"),
                "fan_speed" => final_property = String::from("fan.speed"),
                "power_usage" => final_property = String::from("power.draw"),
                _ => return Err(String::from("Unknown property.."))
            },
            // Nvidia Optimus
            3 => match property {
                "name" => final_property = String::from("gpu_name"),
                "util" => final_property = String::from("utilization.gpu"),
                "temp" => final_property = String::from("temperature.gpu"),
                "memory_usage" => final_property = String::from("memory.used"),
                "memory_total" => final_property = String::from("memory.total"),
                "fan_speed" => final_property = String::from("fan.speed"),
                "power_usage" => final_property = String::from("power.draw"),
                _ => return Err(String::from("Unknown property.."))
            },
            // ???
            _ => return Err(String::from("Unknown provider type.."))
        }

        // Grab relevant property
        for prop in self.imp().properties.borrow().iter() {
            //println!("current property: `{}`", prop.property::<String>("id"));//TEST
            //println!("looking for property: `{}`", final_property);//TEST

            if prop.property::<String>("id") == final_property {
                // Run and return output
                match prop.to_owned().parse(uuid) {
                    Some(stat) => return Ok(stat),
                    None => return Err(String::from("Problem occured when trying to run property..")),
                }
            }
        }

        Err(String::from("Cannot find property.."))
    }

    /**
     * Name:
     * open_settings
     *
     * Description:
     * Open settings if provider is capable
     *
     * Made:
     * 24/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn open_settings(&self) -> Result<(), String> {
        // Check provider type
        match self.property::<i32>("provider-type") {
            // Open Nvidia Settings
            0 => {
                match exec_check(
                    &[OsStr::new("nvidia-settings")],
                    None::<&gio::Cancellable>,
                ) {
                    Ok(result) => return Ok(result),
                    Err(err) => return Err(String::from(err.message())),
                };
            }
            1 => {
                match exec_check(
                    &[OsStr::new("nvidia-settings")],
                    None::<&gio::Cancellable>,
                ) {
                    Ok(result) => return Ok(result),
                    Err(err) => return Err(String::from(err.message())),
                };
            }
            // Error Message
            2 => Err(String::from("Nvidia Settings is not enabled in preferences..")),
            3 => Err(String::from("Nvidia Settings is not enabled in preferences..")),
            _ => Err(String::from("Invalid provider, check preferences..")),
        }
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
        Self::new(Vec::new, 0)
    }
}
