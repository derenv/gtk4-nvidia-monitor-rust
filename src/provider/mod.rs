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

// Imports
use adwaita::{gio, glib};
use glib::Object;
use gtk::prelude::*;
use std::ffi::OsStr;

// Crates
use crate::{processor::Processor, property::Property, subprocess};

// GObject wrapper for Provider
glib::wrapper! {
    pub struct Provider(ObjectSubclass<imp::Provider>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
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
        let properties: Vec<Property> = func();
        if !properties.is_empty() {
            obj.set_property("utilization-property", properties[0].clone());
            obj.set_property("temperature-property", properties[1].clone());
            obj.set_property("memory-usage-property", properties[2].clone());
            obj.set_property("fan-speed-property", properties[3].clone());
            if properties.len() == 5 {
                // Only gets set when smi is present
                obj.set_property("power-usage-property", properties[4].clone());
            }
        }

        obj
    }

    /*
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
        );

        // Validate output
        match processor.process() {
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

    /*
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
     */
    pub fn get_gpu_data(&self, uuid: &str, property: &str) -> Result<String, String> {
        // Check provider type
        let processor_args: [String; 2];
        match self.property::<i32>("provider-type") {
            // Nvidia Settings+SMI / Nvidia SMI
            0 | 2 => {
                match property {
                    "name" => {
                        processor_args = [
                            String::from("nvidia-smi"),
                            String::from("--query-gpu=gpu_name --format=csv,noheader -i ") + uuid
                        ];
                    }
                    _ => {
                        // Return error..
                        return Err(String::from("Invalid property name, check provider preferences.."))
                    }
                }
            }
            // Nvidia Optimus
            3 => {
                match property {
                    "name" => {
                        processor_args = [
                            String::from("optirun"),
                            String::from("nvidia-smi --query-gpu=gpu_name --format=csv,noheader -i ") + uuid
                        ];
                    }
                    _ => {
                        // Return error..
                        return Err(String::from("Invalid property name, check provider preferences.."))
                    }
                }
            }
            _ => {
                // Return error..
                return Err(String::from("Invalid provider, check preferences.."))
            }
        }

        // Create a processor object with appropriate args
        let processor: Processor = Processor::new(
            &processor_args[0],
            &processor_args[1],
        );

        // Validate output
        match processor.process() {
            Ok(output) => match output {
                Some(valid_output) => {
                    // If a valid output given, check if correct length (1)
                    match valid_output.len() {
                        1 => return Ok(String::from(valid_output[0].as_str())),
                        _ => return Err(String::from("Process encountered an unknown error.."))
                    }
                }
                None => return Err(String::from("Process encountered an unknown error.."))
            },
            Err(err) => return Err(String::from(err.message()))
        }
    }

    /*
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
                match subprocess::exec_check(
                    &[OsStr::new("nvidia-settings")],
                    None::<&gio::Cancellable>,
                ) {
                    Ok(result) => return Ok(result),
                    Err(err) => return Err(String::from(err.message())),
                };
            }
            1 => {
                match subprocess::exec_check(
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

    /*
     * Name:
     * update_property_value
     *
     * Description:
     * Updates the internal values of all the GPU properties
     *
     * Made:
     * 28/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn update_property_value<T: ToValue + std::marker::Copy>(
        &self,
        property_name: &str,
        value: T
    ) -> Result<(),String>{
        // Fetch the list of properties (dependant on provider type)
        match self.property::<i32>("provider-type") {
            1 => {
                // Fetch list
                let properties = vec![
                    self.property::<Property>("utilization-property"),
                    self.property::<Property>("temperature-property"),
                    self.property::<Property>("memory-usage-property"),
                    self.property::<Property>("fan-speed-property"),
                ];

                // Update value of property property (lol) in each
                for prop in properties {
                    // Update internal value, if this fails there is a panic
                    prop.update_value(property_name, value);
                }

                // Return sucess
                Ok(())
            }
            _ => {
                // Fetch list
                let properties: [Property; 5] = [
                    self.property::<Property>("utilization-property"),
                    self.property::<Property>("temperature-property"),
                    self.property::<Property>("memory-usage-property"),
                    self.property::<Property>("fan-speed-property"),
                    self.property::<Property>("power-usage-property"),
                ];

                // Update value of property property (lol) in each
                for prop in properties {
                    // Update internal value, if this fails there is a panic
                    prop.update_value(property_name, value);
                }

                // Return sucess
                Ok(())
            }
        }
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
        Self::new(Vec::new, 0)
    }
}
