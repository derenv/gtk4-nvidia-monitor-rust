<!--
SPDX-FileCopyrightText: 2022 Deren Vural
SPDX-License-Identifier: GPL-3.0-or-later
-->

# Description

This project is a rewrite of gnome-nvidia-extension in Rust - intended as a practise Rust project.

# Usage

Use the following to compile the project:

```bash
cargo clean; cargo run
```

# Resources

## Original Extension

[Original version, written in GTK-JS](https://github.com/ethanwharris/gnome-nvidia-extension)

## Documentation

- [GTK-RS Project website](https://gtk-rs.org/)
- [GTK-RS Book](https://gtk-rs.org/gtk4-rs/stable/latest/book/introduction.html)
- [GTK-RS Documentation](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/)
- [GLib Documentation](https://docs.gtk.org/glib/index.html)
- [GObject Documentation](https://docs.gtk.org/gobject/index.html)

## GitHub

- [GTK-RS Organisation on GitHub](https://github.com/gtk-rs)
- [GTK-RS Core](https://github.com/gtk-rs/gtk-rs-core)
- [GTK3-RS](https://github.com/gtk-rs/gtk3-rs)
- [GTK4-RS](https://github.com/gtk-rs/gtk4-rs)

## Crates

### GTK libraries

- [GTK4](https://crates.io/crates/gtk4)
- [GLib](https://crates.io/crates/glib/0.15.12)
- [Gio](https://crates.io/crates/gio/0.15.12)

### System Tray Library

[libappindicator](https://crates.io/crates/libappindicator/0.7.1)

There are other ways of doing this - libappindicator seems broken..

## Examples

- [GObject Subclassing (GTK3)](https://github.com/gtk-rs/gtk3-rs/tree/master/examples/basic_subclass)
- [GObject Subclassing (GTK4)](https://github.com/gtk-rs/gtk4-rs/tree/8b0c4cbd35912b9f8685d40f796b0806c52119ab/book/listings/todo/1)
- [System Tray (libappindicator)](https://github.com/tauri-apps/libappindicator-rs/blob/main/examples/hello.rs)

## System Tray Discussion

[Reddit Post](https://www.reddit.com/r/gnome/comments/7x7qc6/by_what_logic_was_system_tray_removed/)

The post above has possible alternatives to libappindicator in the comments

# License

[![License: GPL v3.0+](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)
