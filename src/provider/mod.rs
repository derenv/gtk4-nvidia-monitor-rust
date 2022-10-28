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
use std::ffi::OsStr;
use glib::Object;
use adwaita::{glib, gio};
use gtk::prelude::*;

// Crates
use crate::{property::Property, subprocess, processor::Processor};

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

        obj.set_property("provider-type", provider_type);

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
        match self.property::<i32>("provider-type") {
            // Nvidia Settings/SMI
            0 => {
                // Create a processor object with appropriate args
                let processor: Processor = Processor::new("nvidia-settings", "-q GpuUUID -t");

                // Validate output
                match processor.process() {
                    Ok(output) => match output {
                        Some(valid_output) => {
                            // If a valid output given, finally return to main window
                            return Ok(valid_output);
                        },
                        None => {
                            // Return error..
                            return Err("Process encountered an unknown error..".to_string());
                        },
                    },
                    Err(err) => {
                        // Return error..
                        return Err(err.message().to_owned());
                    },
                }
            },
            // Nvidia Settings
            1 => {
                // Create a processor object with appropriate args
                let processor: Processor = Processor::new("nvidia-settings", "-q GpuUUID -t");

                // Validate output
                match processor.process() {
                    Ok(output) => match output {
                        Some(valid_output) => {
                            // If a valid output given, finally return to main window
                            return Ok(valid_output);
                        },
                        None => {
                            // Return error..
                            return Err("Process encountered an unknown error..".to_string());
                        },
                    },
                    Err(err) => {
                        // Return error..
                        return Err(err.message().to_owned());
                    },
                }
            },
            // Nvidia SMI
            2 => {
                // Create a processor object with appropriate args
                let processor: Processor = Processor::new("nvidia-smi", "--query-gpu=gpu_name --format=csv,noheader");

                // Validate output
                match processor.process() {
                    Ok(output) => match output {
                        Some(valid_output) => {
                            // If a valid output given, finally return to main window
                            return Ok(valid_output);
                        },
                        None => {
                            // Return error..
                            return Err("Process encountered an unknown error..".to_string());
                        },
                    },
                    Err(err) => {
                        // Return error..
                        return Err(err.message().to_owned());
                    },
                }
            },
            // Nvidia Optimus
            3 => {
                // Create a processor object with appropriate args
                let processor: Processor = Processor::new("optirun", "nvidia-smi --query-gpu=gpu_name --format=csv,noheader");

                // Validate output
                match processor.process() {
                    Ok(output) => match output {
                        Some(valid_output) => {
                            // If a valid output given, finally return to main window
                            return Ok(valid_output);
                        },
                        None => {
                            // Return error..
                            return Err("Process encountered an unknown error..".to_string());
                        },
                    },
                    Err(err) => {
                        // Return error..
                        return Err(err.message().to_owned());
                    },
                }
            },
            _ => {
                // Return error..
                Err("Invalid provider, check preferences..".to_string())
            },
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
    pub fn open_settings(&self) -> Result<(), &str> {
        // Check provider type
        match self.property::<i32>("provider-type") {
            // Open Nvidia Settings
            0 => {
                match subprocess::exec_check(&[OsStr::new("nvidia-settings")], None::<&gio::Cancellable>) {
                    Ok(result) => {
                        Ok(result)
                    },
                    Err(err) => {
                        Err(err.message())
                    },
                };

                Err("Something has gone very wrong..")
            },
            1 => {
                match subprocess::exec_check(&[OsStr::new("nvidia-settings")], None::<&gio::Cancellable>) {
                    Ok(result) => {
                        Ok(result)
                    },
                    Err(err) => {
                        Err(err.message())
                    },
                };

                Err("Something has gone very wrong..")
            },
            // Error Message
            2 => {
                Err("Nvidia Settings is not enabled in preferences..")
            },
            3 => {
                Err("Nvidia Settings is not enabled in preferences..")
            },
            _ => {
                Err("Invalid provider, check preferences..")
            },
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
        Self::new(|| Vec::new(), 0)
    }
}
