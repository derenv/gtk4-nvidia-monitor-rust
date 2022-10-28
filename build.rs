// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * build.rs
 *
 * Description:
 * Build script for Nvidia Gnome Extension (Rust)
 *
 * Made:
 * 09/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * imports must be added as a dependancy for build scripts via:
 *     `cargo add gtk4 --rename gtk --build`
 * which updates Cargo.toml with:
 *     [build-dependencies]
 *     gtk = { version = "^0.4.8", package = "gtk4" }
 */

// Imports
use adwaita::gio::compile_resources;

/*
 * Name:
 * main
 *
 * Description:
 * Runs pre-build
 *
 * Made:
 * 09/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
fn main() {
    // UI
    println!("..Compiling UI resources into `.gresource` file");
    compile_resources(
        "src/resources",
        "src/resources/resources.gresource.xml",
        "nvidiamonitorrust.gresource",
    );
}
