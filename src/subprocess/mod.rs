// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing subprocess module
 *
 * Made:
 * 15/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * https://blog.logrocket.com/a-practical-guide-to-async-in-rust/
 */

// Imports
use std::ffi::OsStr;
use gtk::{ gio, glib };
use gtk::prelude::*;

/*
 * Name:
 * execCheck
 *
 * Description:
 * Execute a command asynchronously and check the exit status
 *
 * If given, @cancellable can be used to stop the process before it finishes.
 *
 * https://gtk-rs.org/gtk-rs-core/stable/0.14/docs/src/gio/auto/subprocess.rs.html
 *
 * Made:
 * 15/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * @param {string[]} argv - a list of string arguments
 * @param {Gio.Cancellable} [cancellable] - optional cancellable object
 * @returns {Promise<boolean>} - The process success
 *
 */
pub fn exec_check<'a>(argv: &[&OsStr], cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<(), glib::Error> {
    // Create subprocess
    println!("..creating subprocess");//DEBUG
    match gio::Subprocess::newv(argv, gio::SubprocessFlags::NONE) {
        Err(err) => {
            Err(err)
        },
        Ok(proc) => {
            println!("..Subprocess successfully created!");//DEBUG

            // Run subprocess

            // Define callback
            //This should be called when the process is done
            /*
            let callback = |q: Result<(), glib::Error>| {
                match q {
                    Err(err) => {
                        Err(err)
                    },
                    Ok(_out) => {
                        println!("....process finished");//DEBUG
                        Ok(())
                    },
                }
            };

            struct Listener {
                done: bool
            }
            impl Listener {
                pub fn on_call(&mut self, done: bool) { self.done = done }
            }
            struct Caller<'callback> {
                callback: Box<dyn FnMut(bool) + 'callback>,
            }
            impl Caller<'_> {
                pub fn call(&mut self) { (self.callback)(true) }
            }
            let mut listener = Listener { done: false };
            let mut caller = Caller { callback: Box::new(|x| listener.on_call(x)) };

            //fn callback_fn(x: bool){println!("callback bitch!! {}", x)}
            //let callback_box = Box::new(|x: bool| callback_fn(x));

            //proc.wait_future();//not sure what this does tbh
            */

            /*
            Do i just not use Results with callback versions?
            */
            // This doesn't work
            /*
            match proc.wait_async(cancellable,None) {
                Err(err) => {
                    Err(err)
                },
                Ok(_out) => {
                    println!("....process finished");//DEBUG
                    Ok(())
                },
            }
            */
            // This also doesn't work
            /*
            match proc.wait_async(cancellable, callback) {
                Err(err) => {
                    Err(err)
                },
                Ok(_out) => {
                    println!("....process finished");//DEBUG
                    Ok(())
                },
            }
            */

            //*
            // This works but holds up main thread..
            match proc.wait(cancellable) {
                Err(err) => {
                    Err(err)
                },
                Ok(_out) => {
                    println!("....process finished");//DEBUG
                    Ok(())
                },
            }
            //*/
        }
    }
}

pub fn exec_communicate<'a>(argv: &[&OsStr], _input: Option<&str>, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<(), glib::Error> {
    // Create subprocess
    println!("..creating subprocess");//DEBUG
    match gio::Subprocess::newv(argv, gio::SubprocessFlags::NONE) {
        Err(err) => {
            Err(err)
        },
        Ok(proc) => {
            println!("..Subprocess successfully created!");//DEBUG

            // Run subprocess
            match proc.communicate(None, cancellable) {
                Err(err) => {
                    Err(err)
                },
                Ok(_out) => {
                    println!("....process finished");//DEBUG
                    Ok(())
                },
            }
        }
    }
}
