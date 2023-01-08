// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (GpuPage)
 *
 * Made:
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */

// Imports
use adwaita::{gio, glib, prelude::*, ViewStack, ViewSwitcherBar};
use gio::Settings;
use glib::{
    once_cell::sync::Lazy, once_cell::sync::OnceCell, subclass::InitializingObject, FromVariant,
    ParamSpec, ToValue, Value, subclass::Signal, subclass::SignalType
};
use gtk::{subclass::prelude::*, CompositeTemplate, TemplateChild};
use std::{cell::Cell, cell::RefCell, rc::Rc};

// Modules
use crate::{modificationwindow::ModificationWindow, provider::Provider};

/// Structure for storing a SettingsWindow object and any related information
#[derive(Default)]
pub struct ModificationWindowContainer {
    pub window: Option<ModificationWindow>,
    pub open: bool,
}

/// Object holding the State and any Template Children
#[derive(CompositeTemplate, Default)]
#[template(resource = "/gpu-page.ui")]
pub struct GpuPage {
    pub settings: OnceCell<Settings>,
    uuid: OnceCell<String>,
    name: OnceCell<String>,
    provider: OnceCell<Option<Provider>>,
    refreshid: Cell<u32>,

    pub modification_window: Rc<RefCell<ModificationWindowContainer>>,

    #[template_child]
    pub view_switcher: TemplateChild<ViewSwitcherBar>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GpuPage {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NvidiaExtensionGpuPage";
    type Type = super::GpuPage;
    type ParentType = gtk::Grid;

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
 * GpuPage
 *
 * Description:
 * Trait shared by all GpuPage objects
 *
 * Made:
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
#[gtk::template_callbacks]
impl GpuPage {
    //
}

impl GpuPage {
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

    /**
     * Name:
     * replace_stack
     *
     * Description:
     * Replace current view_stack using passed value
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
    pub fn replace_stack(&self, stack: Option<&ViewStack>) {
        self.view_switcher.set_stack(stack);
    }
}

/**
 * Trait Name:
 * ObjectImpl
 *
 * Description:
 * Trait shared by all GObjects
 *
 * Made:
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ObjectImpl for GpuPage {
    /**
     * Name:
     * constructed
     *
     * Description:
     * Called during construction, allows calling setup functions
     *
     * Made:
     * 02/11/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn constructed(&self, obj: &Self::Type) {
        // println!("CONSTRUCTED");//TEST
        // Call "constructed" on parent
        self.parent_constructed(obj);

        // Setup
        self.refreshid.set(0);
        //obj.setup_settings();
        //obj.load_properties();//TODO
        //obj.setup_widgets();
        //obj.setup_callbacks();
        //obj.setup_actions();
    }

    /**
     * Name:
     * properties
     *
     * Description:
     * Create list of custom properties for our GObject
     *
     * Made:
     * 02/11/2022
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
        //println!("PROPERTIES");//TEST
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::builder("uuid").build(),
                glib::ParamSpecString::builder("name").build(),
                glib::ParamSpecObject::builder("provider", glib::Type::OBJECT).build(),
                glib::ParamSpecUInt::builder("refreshid").build(),
            ]
        });

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
     * 02/11/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        //println!("setting: {:?}", pspec.name());//TEST

        // println!("setting: {:?}", pspec.name());//TEST
        // let x: String = self.uuid.take();
        // self.uuid.set(x.clone());
        // println!("U: `{}`", x);
        // let x: String = self.name.take();
        // self.name.set(x.clone());
        // println!("N: `{}`", x);

        match pspec.name() {
            "uuid" => match value.get() {
                Ok(input_uuid) => self
                    .uuid
                    .set(input_uuid)
                    .expect("`uuid` should not be set after calling constructor.."),
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "name" => match value.get() {
                Ok(input_name) => self
                    .name
                    .set(input_name)
                    .expect("`name` should not be set after calling constructor.."),
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "provider" => match value.get() {
                Ok(input_provider) => self
                    .provider
                    .set(input_provider)
                    .expect("`provider` should not be set after calling constructor.."),
                Err(_) => panic!("The value needs to be of type `Provider`."),
            },
            "refreshid" => match value.get() {
                Ok(input_refreshid_property) => {
                    self.refreshid.replace(input_refreshid_property);
                }
                Err(_) => panic!("The value needs to be of type `u32`."),
            },
            _ => panic!("Property `{}` does not exist..", pspec.name()),
        }

        // let x: String = self.uuid.take();
        // self.uuid.set(x.clone());
        // println!("U: `{}`", x);
        // let x: String = self.name.take();
        // self.name.set(x.clone());
        // println!("N: `{}`", x);
    }

    /**
     * Name:
     * property
     *
     * Description:
     * Accessor for custom GObject properties
     *
     * Made:
     * 02/11/2022
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
            "uuid" => match self.uuid.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `uuid` property.."),
            },
            "name" => match self.name.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `name` property.."),
            },
            "provider" => match self.provider.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `provider` property.."),
            },
            "refreshid" => {
                let value: u32 = self.refreshid.take();

                self.refreshid.set(value.clone());

                value.to_value()
            }
            _ => panic!("Property `{}` does not exist..", pspec.name()),
        }
    }

    /**
     * Name:
     * signals
     *
     * Description:
     * Defines the list of signals
     *
     * Made:
     * 07/01/2023
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * beware that you need to use kebab-case (<https://en.wikipedia.org/wiki/Letter_case#Kebab_case>)
     *
     * <https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_signals.html>
     *
     * SignalType::from(i32::static_type())
     */
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("update-views",
                    &[SignalType::from(i32::static_type())],
                    SignalType::from(i32::static_type()))
                .build()
            ]
        });
        SIGNALS.as_ref()
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
 * 09/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl WidgetImpl for GpuPage {}

/**
 * Trait Name:
 * GridImpl
 *
 * Description:
 * Trait shared by all grids
 *
 * Made:
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl GridImpl for GpuPage {}
