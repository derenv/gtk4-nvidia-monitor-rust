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
use adwaita::{gio, glib, prelude::*};
use gio::Settings;
use glib::{
    once_cell::sync::Lazy, once_cell::sync::OnceCell, subclass::InitializingObject, ParamSpec,
    ToValue, Value,
};
use gtk::{subclass::prelude::*, CompositeTemplate};
use std::{cell::Cell, cell::Ref, cell::RefCell, rc::Rc};

// Modules
use crate::provider::Provider;

/// Object holding the State and any Template Children
#[derive(CompositeTemplate, Default)]
#[template(resource = "/gpu-page.ui")]
pub struct GpuPage {
    pub settings: OnceCell<Settings>,
    uuid: Cell<String>,
    name: Cell<String>,
    provider: Rc<RefCell<Provider>>,
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
    // #[template_callback]
    // fn get_gpu_data(&self, _label: &Label) {
    //     //
    //     println!("TEST callback");//TEST
    // }
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
                Ok(input_uuid) => {
                    self.uuid.replace(input_uuid);
                }
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "name" => match value.get() {
                Ok(input_name) => {
                    self.name.replace(input_name);
                }
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "provider" => match value.get() {
                Ok(input_provider_property) => {
                    self.provider.replace(input_provider_property);
                }
                Err(_) => panic!("The value needs to be of type `Provider`."),
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
            "uuid" => {
                //TODO: this seems ridiculous..
                let value: String = self.uuid.take();

                self.uuid.set(value.clone());

                value.to_value()
            }
            "name" => {
                //TODO: this seems ridiculous..
                let value: String = self.name.take();

                self.name.set(value.clone());

                value.to_value()
            }
            "provider" => {
                let value: Ref<Provider> = self.provider.borrow();

                //self.provider.set(value.clone());

                value.to_value()
            }
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
