// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

use gtk::gio;

fn main() {
    // UI
    println!("compiling UI resources");
    gio::compile_resources(
        "src/resources",
        "src/resources/resources.gresource.xml",
        "nvidiaextensionrust.gresource",
    );
}
