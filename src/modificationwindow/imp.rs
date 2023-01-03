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
use gtk::{subclass::prelude::*, CheckButton, CompositeTemplate, TemplateChild};
use std::cell::Cell;

// Modules
//

/// Structure for storing a View item data
#[derive(Default, Clone)]
pub struct ViewComponent {
    pub name: String,
    pub position: i32,
}

/// Object holding the State and any Template Children
#[derive(CompositeTemplate, Default)]
#[template(resource = "/modification-window.ui")]
pub struct ModificationWindow {
    pub settings: OnceCell<Settings>,
    pub uuid: OnceCell<String>,

    pub old_view_id: OnceCell<i32>,
    pub new_view_id: Cell<i32>,
    pub old_view_title: OnceCell<String>,
    pub new_view_title: Cell<String>,

    pub view_components_list: Cell<Vec<ViewComponent>>,

    #[template_child]
    pub util_checkbox: TemplateChild<CheckButton>,
    #[template_child]
    pub temp_checkbox: TemplateChild<CheckButton>,
    #[template_child]
    pub power_usage_checkbox: TemplateChild<CheckButton>,
    #[template_child]
    pub mem_usage_checkbox: TemplateChild<CheckButton>,
    #[template_child]
    pub mem_total_checkbox: TemplateChild<CheckButton>,
    #[template_child]
    pub mem_util_checkbox: TemplateChild<CheckButton>,
    #[template_child]
    pub fan_speed_checkbox: TemplateChild<CheckButton>,
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
     * update_view_components_list
     *
     * Description:
     * Update the stored properties for this view
     *
     * Made:
     * 02/01/2023
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn update_stored_data(&self) {
        // Get stored & const view data
        let mut stored_views_data: Vec<String> = self.get_setting::<Vec<String>>("viewconfigs");
        let mut stored_views_components: Vec<String> =
            self.get_setting::<Vec<String>>("viewcomponentconfigs");
        let uuid: String = self
            .uuid
            .clone()
            .get()
            .expect("missing `uuid`..")
            .to_owned();
        println!("stored views: `{:?}`", stored_views_data); //TEST

        // Get old + new view title
        let old_view_title: String = self
            .old_view_title
            .clone()
            .get()
            .expect("missing `old-view-title`..")
            .to_owned();
        let new_view_title: String = self.new_view_title.take();
        self.new_view_title.set(new_view_title.clone());

        // If present in saved settings, use! otherwise follow below defaults
        if stored_views_data.len() == 0 {
            if stored_views_components.len() != 0 {
                panic!("something really wrong!");
            }

            // Create new viewconfig
            //UUID:POSITION:VIEW_TITLE
            let new_viewconfig: String = String::from(&uuid) + ":0:" + &new_view_title;

            // Save new viewconfig
            self.update_setting::<Vec<String>>("viewconfigs", vec![new_viewconfig]);

            // Get current components
            let current_components: Vec<ViewComponent> = self.view_components_list.take();

            // Create new viewcomponentconfigs
            let mut final_viewcomponentconfigs: Vec<String> = vec![];
            // For each selected property component
            for component in current_components {
                //UUID:VIEW_TITLE:POSITION:name
                let formatted_component: String = String::from(&uuid)
                    + ":"
                    + &new_view_title
                    + ":"
                    + &component.position.to_string()
                    + ":"
                    + &component.name;

                // Add to final list
                final_viewcomponentconfigs.push(formatted_component);
            }

            // Save new viewcomponentconfigs
            self.update_setting::<Vec<String>>("viewcomponentconfigs", final_viewcomponentconfigs);
        } else {
            // index used to tell if this is a new viewconfig or not
            let mut component_index: i32 = -1;

            // Get list of stored viewconfigs
            for index in 0..stored_views_data.len() {
                // Split current viewconfig
                let sub_items: Vec<&str> = stored_views_data[index].split(':').collect();

                // If viewconfig is for this GPU (i.e. has valid UUID) and has the old name
                if (sub_items[0] == uuid, sub_items[2] == old_view_title) == (true, true) {
                    component_index = index as i32;
                    println!("match.."); //TEST
                    break;
                }
            }

            // If we are modifying an existing viewconfig
            if component_index != -1 {
                // Get old + new view id
                let old_view_id: i32 = self
                    .old_view_id
                    .clone()
                    .get()
                    .expect("missing `old-view-id`..")
                    .to_owned();
                let new_view_id: i32 = self.new_view_id.take();
                self.new_view_id.set(new_view_id.clone());

                // Update viewconfigs accordingly
                match (old_view_title == new_view_title, old_view_id == new_view_id) {
                    // MATCH name is different, id is the same
                    (false, true) => {
                        // Remove old viewconfig
                        stored_views_data.remove(component_index as usize);

                        // Create new viewconfig
                        //UUID:POSITION:VIEW_TITLE
                        let new_viewconfig: String =
                            uuid.clone() + ":" + &old_view_id.to_string() + ":" + &new_view_title;

                        // Update viewconfigs item with new viewconfig
                        stored_views_data.push(new_viewconfig);

                        // Update stored viewconfigs
                        self.update_setting::<Vec<String>>("viewconfigs", stored_views_data);
                        println!("viewconfig updated.."); //TEST
                    }
                    // MATCH name is different, id is different
                    (false, false) => {
                        // Remove old viewconfig
                        stored_views_data.remove(component_index as usize);

                        // Create new viewconfig
                        //UUID:POSITION:VIEW_TITLE
                        let new_viewconfig: String =
                            uuid.clone() + ":" + &new_view_id.to_string() + ":" + &new_view_title;

                        // Update viewconfigs item with new viewconfig
                        stored_views_data.push(new_viewconfig);

                        // Update stored viewconfigs
                        self.update_setting::<Vec<String>>("viewconfigs", stored_views_data);
                        println!("viewconfig updated.."); //TEST
                    }
                    // MATCH name is the same, id is different
                    (true, false) => {
                        // Remove old viewconfig
                        stored_views_data.remove(component_index as usize);

                        // Create new viewconfig
                        //UUID:POSITION:VIEW_TITLE
                        let new_viewconfig: String =
                            uuid.clone() + ":" + &new_view_id.to_string() + ":" + &old_view_title;

                        // Update viewconfigs item with new viewconfig
                        stored_views_data.push(new_viewconfig);

                        // Update stored viewconfigs
                        self.update_setting::<Vec<String>>("viewconfigs", stored_views_data);
                        println!("viewconfig updated.."); //TEST
                    }
                    // MATCH name is the same, id is the same
                    (true, true) => {
                        // Do nothing
                        println!("No viewconfig to update.."); //TEST
                    }
                }

                // Create empty final list of viewcomponentconfigs
                let mut new_components_list: Vec<String> = vec![];
                println!("Initial components list: `{:?}`", stored_views_components); //TEST
                let mut changes: bool = false;

                // Check if any new viewcomponentconfigs need updated
                for index in 0..stored_views_components.len() {
                    // Split current viewcomponentconfig
                    let sub_items: Vec<&str> = stored_views_components[index].split(':').collect();

                    // If viewcomponentconfig from this view
                    if (sub_items[0] == &uuid, sub_items[1] == old_view_title) == (true, true) {
                        // Get current components
                        let current_components: Vec<ViewComponent> =
                            self.view_components_list.take();

                        // for each new possible viewcomponentconfig
                        for c_index in 0..current_components.len() {
                            //for component in current_components {
                            // If it is the same viewcomponentconfig
                            match (
                                current_components[c_index].name == sub_items[3],
                                old_view_title == new_view_title,
                                current_components[c_index].position.to_string() == sub_items[2],
                            ) {
                                // CORRECT NAME, TITLE CHANGE, POSITION CHANGE
                                (true, false, false) => {
                                    // Create new viewcomponentconfig
                                    let new_viewcomponentconfig: String = uuid.clone()
                                        + ":"
                                        + &new_view_title
                                        + ":"
                                        + &current_components[c_index].position.to_string()
                                        + ":"
                                        + &current_components[c_index].name;

                                    // Add to updated list of stored viewcomponentconfigs
                                    new_components_list.push(new_viewcomponentconfig);
                                    changes = true;
                                }
                                // CORRECT NAME, TITLE CHANGE, NO POSITION CHANGE
                                (true, false, true) => {
                                    // Create new viewcomponentconfig
                                    let new_viewcomponentconfig: String = uuid.clone()
                                        + ":"
                                        + &new_view_title
                                        + ":"
                                        + sub_items[2]
                                        + ":"
                                        + &current_components[c_index].name;

                                    // Add to updated list of stored viewcomponentconfigs
                                    new_components_list.push(new_viewcomponentconfig);
                                    changes = true;
                                }

                                // CORRECT NAME, NO TITLE CHANGE, POSITION CHANGE
                                (true, true, false) => {
                                    // Create new viewcomponentconfig
                                    let new_viewcomponentconfig: String = uuid.clone()
                                        + ":"
                                        + &old_view_title
                                        + ":"
                                        + &current_components[c_index].position.to_string()
                                        + ":"
                                        + &current_components[c_index].name;

                                    // Add to updated list of stored viewcomponentconfigs
                                    new_components_list.push(new_viewcomponentconfig);
                                    changes = true;
                                }

                                // NO CHANGES OR INCORRECT NAME, IGNORE
                                (true, true, true) | (false, _, _) => {
                                    // Add to updated list of stored viewcomponentconfigs
                                    new_components_list
                                        .push(stored_views_components[index].clone());
                                }
                            }
                        }

                        // Put our components back for next loop
                        self.view_components_list.set(current_components);
                    } else {
                        // Add to updated list of stored viewcomponentconfigs
                        new_components_list.push(stored_views_components[index].clone());
                    }
                }

                // Update stored viewcomponentconfigs
                println!("Final components list: `{:?}`", stored_views_components);
                if changes {
                    self.update_setting::<Vec<String>>("viewcomponentconfigs", new_components_list);
                    println!("saving changes.."); //TEST
                } else {
                    println!("no changes.."); //TEST
                }
            } else {
                // Create new viewconfig
                //UUID:POSITION:VIEW_TITLE
                let new_viewconfig: String = uuid.clone() + ":0:" + &new_view_title;

                // Update viewconfigs item with new viewconfig
                stored_views_data.push(new_viewconfig);

                // Update stored viewconfigs
                self.update_setting::<Vec<String>>("viewconfigs", stored_views_data);

                // Get current components
                let current_components: Vec<ViewComponent> = self.view_components_list.take();

                // Create new viewcomponentconfigs
                // For each selected property component
                for component in current_components {
                    //UUID:VIEW_TITLE:POSITION:name
                    let formatted_component: String = uuid.clone()
                        + ":"
                        + &new_view_title
                        + ":"
                        + &component.position.to_string()
                        + ":"
                        + &component.name;

                    // Add to list
                    stored_views_components.push(formatted_component);
                }

                // Update stored viewconfigs
                println!("Final components list: `{:?}`", stored_views_components);
                self.update_setting::<Vec<String>>("viewcomponentconfigs", stored_views_components);
            }
        }
    }

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
impl ModificationWindow {
    /**
     * Name:
     * util_property_selected
     *
     * Description:
     * Template callback for setting properties shown in current view
     *
     * Made:
     * 02/01/2023
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    #[template_callback]
    fn util_property_selected(&self, button: &CheckButton) {
        /*
        let title: String = self.view_title.take();
        self.view_title.set(title.clone());

        let new_item: String = self.uuid.clone().take().unwrap()
                            + ":"
                            + &title
                            + ":"
                            + &self.view_id.clone().take().unwrap().to_string()
                            + ":util";

        let list: Vec<GString> = self.view_components_list.take();
        list.push(new_item.into());
        */
    }
    #[template_callback]
    fn temp_property_selected(&self, button: &CheckButton) {
        // Create new item
        let new_item: ViewComponent = ViewComponent {
            name: String::from("temp"),
            position: -1,
        };

        // Add to list of items
        //

        //NOTE: NEEDS A TEXTBOX FOR VIEW NAME
        //NOTE: NEEDS A SCROLLWHEEL FOR # OF VIEW POSITIONS
        //NOTE: SWITCH THESE CHECKBUTTONS TO DROP-DOWN MENU FOR EACH POSITION
        //^THIS WAY DON'T NEED TO SET POSITION SEPERATELY

        // Activate scroll wheel (for position)
        //

        //let list: Vec<GString> = self.view_components_list.take();
        //list.push(new_item.into());
    }
    #[template_callback]
    fn power_usage_property_selected(&self, button: &CheckButton) {}
    #[template_callback]
    fn mem_usage_property_selected(&self, button: &CheckButton) {}
    #[template_callback]
    fn mem_total_property_selected(&self, button: &CheckButton) {}
    #[template_callback]
    fn mem_util_property_selected(&self, button: &CheckButton) {}
    #[template_callback]
    fn fan_speed_property_selected(&self, button: &CheckButton) {}
}

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
        // obj.setup_settings();
        // obj.setup_widgets();
        // obj.restore_data();
        // obj.setup_callbacks();
        // obj.setup_actions();
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
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecInt::builder("old-view-id").build(),
                glib::ParamSpecInt::builder("new-view-id").build(),
                glib::ParamSpecString::builder("old-view-title").build(),
                glib::ParamSpecString::builder("new-view-title").build(),
                glib::ParamSpecString::builder("uuid").build(),
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
            "old-view-id" => match value.get() {
                Ok(input_old_view_id) => self
                    .old_view_id
                    .set(input_old_view_id)
                    .expect("`old-view-id` should not be set after calling constructor.."),
                Err(_) => panic!("The value needs to be of type `i32`."),
            },
            "new-view-id" => match value.get() {
                Ok(input_new_view_id) => self.new_view_id.set(input_new_view_id),
                Err(_) => panic!("The value needs to be of type `i32`."),
            },
            "old-view-title" => match value.get() {
                Ok(input_old_view_title) => self
                    .old_view_title
                    .set(input_old_view_title)
                    .expect("`old-view-title` should not be set after calling `setup_widgets()`.."),
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "new-view-title" => match value.get() {
                Ok(input_new_view_title) => self.new_view_title.set(input_new_view_title),
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "uuid" => match value.get() {
                Ok(input_uuid) => self
                    .uuid
                    .set(input_uuid)
                    .expect("`uuid` should not be set after calling constructor.."),
                Err(_) => panic!("The value needs to be of type `String`."),
            },
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
            "old-view-id" => match self.old_view_id.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `old-view-id` property.."),
            },
            "new-view-id" => self.new_view_id.get().to_value(),
            "old-view-title" => match self.old_view_title.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `old-view-title` property.."),
            },
            "new-view-title" => {
                let value: String = self.new_view_title.take();

                self.new_view_title.set(value.clone());

                value.to_value()
            }
            "uuid" => match self.uuid.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `uuid` property.."),
            },
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

        // Save any changes to selection here
        self.update_stored_data();

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
