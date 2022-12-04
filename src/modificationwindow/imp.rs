// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (ModificationWindow)
 *
 * Made:
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
// Imports
use adwaita::{gio, glib, prelude::*, subclass::prelude::*};
use gio::Settings;
use glib::{
    once_cell::sync::Lazy, once_cell::sync::OnceCell, signal::Inhibit,
    subclass::InitializingObject, FromVariant, ParamSpec, Value,
};
use gtk::{
    subclass::prelude::*,
    CompositeTemplate,
    //ScrolledWindow, TemplateChild,
};
//use std::{cell::Cell};

// Modules
//

/// Object holding the State and any Template Children
#[derive(CompositeTemplate, Default)]
#[template(resource = "/modification-window.ui")]
pub struct ModificationWindow {
    pub settings: OnceCell<Settings>,
    // #[template_child]
    // left: TemplateChild<Stack>,
    // right: TemplateChild<Stack>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ModificationWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NvidiaExtensionModificationWindow";
    type Type = super::ModificationWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

/**
 * Name:
 * ModificationWindow
 *
 * Description:
 * Trait shared by all ModificationWindow objects
 *
 * Made:
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ModificationWindow {
    /**
     * Name:
     * get_setting
     *
     * Description:
     * Generic function for getting setting value
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn get_setting<T: FromVariant>(&self, name: &str) -> T {
        // Return the value of the property
        match self.settings.get() {
            Some(settings) => settings.get::<T>(name),
            None => panic!("`settings` should be set in `setup_settings`."),
        }
    }

    /**
     * Name:
     * update_setting
     *
     * Description:
     * Generic function for updating setting values
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn update_setting<T: ToVariant>(&self, name: &str, value: T) {
        // Fetch settings
        match self.settings.get() {
            Some(settings) => match settings.set(name, &value) {
                Ok(_) => println!("..Setting `{}` updated!", name),
                Err(err) => panic!("..Cannot update `{}` setting: `{}`", name, err),
            },
            None => panic!("..Cannot retrieve settings"),
        }
    }
}

/**
 * Name:
 * ModificationWindow
 *
 * Description:
 * Trait defining template callbacks shared by all ModificationWindow objects
 *
 * Made:
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
#[gtk::template_callbacks]
impl ModificationWindow {}

/**
 * Trait Name:
 * ObjectImpl
 *
 * Description:
 * Trait shared by all GObjects
 *
 * Made:
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ObjectImpl for ModificationWindow {
    /**
     * Name:
     * constructed
     *
     * Description:
     * Called during construction, allows calling setup functions
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        // Setup
        obj.setup_settings();
        obj.setup_widgets();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_actions();
    }

    /**
     * Name:
     * properties
     *
     * Description:
     * Create list of custom properties for our GObject
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * beware that you need to use kebab-case (<https://en.wikipedia.org/wiki/Letter_case#Kebab_case>)
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

    /**
     * Name:
     * set_property
     *
     * Description:
     * Mutator for custom GObject properties
     *
     * Made:
     * 04/12/2022
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
            _ => panic!("Property `{}` does not exist..", pspec.name()),
        }
    }

    /**
     * Name:
     * property
     *
     * Description:
     * Accessor for custom GObject properties
     *
     * Made:
     * 04/12/2022
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
            _ => panic!("Property `{}` does not exist..", pspec.name()),
        }
    }
}

/**
 * Trait Name:
 * WidgetImpl
 *
 * Description:
 * Trait shared by all widgets
 *
 * Made:
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl WidgetImpl for ModificationWindow {}

/**
 * Trait Name:
 * WindowImpl
 *
 * Description:
 * Trait shared by all Window's
 *
 * Made:
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl WindowImpl for ModificationWindow {
    /*
     * Name:
     * close_request
     *
     * Description:
     * Run when window closed
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn close_request(&self, window: &Self::Type) -> Inhibit {
        // Store state in settings
        self.update_setting("modification-open", false);

        //TODO: Save any changes to selection here
        //

        // Pass close request on to the parent
        self.parent_close_request(window)
    }
}

/**
 * Trait Name:
 * AdwWindowImpl
 *
 * Description:
 * Trait shared by all AdwWindow's
 *
 * Made:
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl AdwWindowImpl for ModificationWindow {}

/**
 * Trait Name:
 * ApplicationWindowImpl
 *
 * Description:
 * Trait shared by all ApplicationWindow's
 *
 * Made:
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ApplicationWindowImpl for ModificationWindow {}

/**
 * Trait Name:
 * AdwApplicationWindowImpl
 *
 * Description:
 * Trait shared by all AdwApplicationWindow's
 *
 * Made:
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl AdwApplicationWindowImpl for ModificationWindow {}
