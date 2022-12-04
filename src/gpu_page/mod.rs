// SPDX-FileCopyrightText: 2022 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (GpuPage)
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
// Custom GObjects
mod imp;
use imp::ModificationWindowContainer;

// Imports
use adwaita::{gio, glib, ViewStack, Application};
use gio::Settings;
use glib::{clone, translate::FromGlib, Object, SourceId};
use gtk::{prelude::*, subclass::prelude::*, Align, Button, Grid, Label, LayoutChild, Orientation};
use std::{sync::Arc, sync::Mutex, cell::RefMut, sync::MutexGuard};

// Modules
use crate::{provider::Provider, modificationwindow::ModificationWindow, APP_ID};

// GObject wrapper for GpuPage
glib::wrapper! {
    pub struct GpuPage(ObjectSubclass<imp::GpuPage>)
    @extends gtk::Grid, gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

/**
 * Trait Name:
 * GpuPage
 *
 * Description:
 * Trait shared by all properties
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
impl GpuPage {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new GpuPage object
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
    pub fn new(uuid: &str, name: &str, provider: Provider) -> Self {
        let obj: GpuPage = Object::new(&[]).expect("Failed to create `GpuPage`.");

        // Set custom properties
        obj.set_property("uuid", String::from(uuid));
        obj.set_property("name", String::from(name));
        obj.set_property("provider", &provider);

        // Apply any setup actions that need the above properties
        obj.setup_settings();
        obj.setup_widgets();

        obj
    }

    /**
     * Name:
     * setup_settings
     *
     * Description:
     * Load settings for APP_ID
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
    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    /**
     * Name:
     * settings
     *
     * Description:
     * Get settings for APP_ID
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
    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    /*
     * Name:
     * load_views
     *
     * Description:
     * Loads all saved views from settings, then calls functions to populate
     *
     * Made:
     * 03/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn load_views(&self) {
        // Get reference to settings object
        let settings_obj: &Settings = self.settings();

        // Load list of Views
        let loaded_views_data: Vec<String> = settings_obj.get::<Vec<String>>("viewconfigs");
        // println!("views saved:`{}`", loaded_views_data.len()); //TEST

        // Remove old content grid
        match self.child_at(0, 0) {
            Some(child) => self.remove(&child),
            None => panic!("unable to find content grid"),
        }

        // Create new content grid
        let content_grid: Grid = Grid::builder()
            .name("content_grid")
            .orientation(Orientation::Vertical)
            .build();
        self.attach(&content_grid, 0, 0 as i32, 1, 1);

        // Fetch grid's layout manager
        match content_grid.layout_manager() {
            Some(grid_manager) => {
                // Create edit button
                let edit_button: Button = Button::builder()
                    .name("edit_button")
                    .label("Edit View")
                    .margin_start(12)
                    .margin_end(12)
                    .margin_top(12)
                    .margin_bottom(12)
                    .halign(Align::Center)
                    .build();
                content_grid.attach(&edit_button, 0, 80, 1, 1);
                edit_button.connect_clicked(clone!(@weak self as gpage => move |_| {
                    // Create modification window
                    // Borrow (mutable) the window's container
                    let mut modification_window_container: RefMut<ModificationWindowContainer> = gpage.imp().modification_window.borrow_mut();

                    // Get state from settings
                    modification_window_container.open = gpage.imp().get_setting::<bool>("modification-open");

                    // Check if an object is stored
                    match &modification_window_container.window {
                        Some(_window) => {
                            println!("..window exists");//DEBUG

                            // Check if the window is already open
                            match modification_window_container.open {
                                false => {
                                    println!("....opening window");//DEBUG

                                    // Create an app object
                                    let app: Application = Application::builder().application_id(APP_ID).build();

                                    // Create new modification window
                                    let new_modification_window: ModificationWindow = ModificationWindow::new(&app);

                                    // Show new modification window
                                    new_modification_window.show();

                                    // Store object and state back in container
                                    modification_window_container.open = true;
                                    modification_window_container.window = Some(new_modification_window);

                                    // Re-create views & properties
                                    //NOTE: any changes are saved to settings on close
                                    gpage.load_views();
                                },
                                true => {
                                    println!("....window already open");//DEBUG
                                },
                            }
                        },
                        None => {
                            println!("..window does not exist");//DEBUG
                            println!("....opening window");//DEBUG

                            // Create an app object
                            let app: Application = Application::builder().application_id(APP_ID).build();

                            // Create modification window
                            let new_modification_window: ModificationWindow = ModificationWindow::new(&app);

                            // Show new modification window
                            new_modification_window.show();

                            // Store object and state back in container
                            modification_window_container.open = true;
                            modification_window_container.window = Some(new_modification_window);

                            // Re-create views & properties
                            //NOTE: any changes are saved to settings on close
                            gpage.load_views();
                        },
                    }

                    // Set new state in settings
                    gpage.imp().update_setting::<bool>("modification-open", modification_window_container.open);
                }));

                // Set layout properties of button
                let child_manager: LayoutChild = grid_manager.layout_child(&edit_button);
                child_manager.set_property("row-span", 2);
                child_manager.set_property("column-span", 2);
            }
            None => panic!("Cannot fetch layout manager of grid.."),
        }

        // Create Views
        // If present in saved settings, use! otherwise follow below defaults
        if let 0 = loaded_views_data.len() {
            // Create default view
            let new_grid_name: String = String::from("default_grid");
            let new_grid: Grid = Grid::builder()
                .name(&new_grid_name)
                .orientation(Orientation::Horizontal)
                //.margin_start(12)
                //.margin_end(12)
                //.margin_top(12)
                //.margin_bottom(12)
                //.halign(Align::Center)
                //.valign(Align::Center)
                //.hexpand(true)
                //.vexpand(true)
                .build();
            content_grid.attach(&new_grid, 0, 0 as i32, 100, 12);

            // Set layout properties of grid
            //let child_manager: LayoutChild = grid_manager.layout_child(&new_grid);
            //child_manager.set_property("row-span", 1);
            //child_manager.set_property("column-span", 1);

            // Build title label & add to grid
            let label_value: String =
                String::from("Please edit the list of Views using 'Edit Views' button");
            let new_title_label: Label = Label::builder()
                .label(&label_value)
                .name("default")
                .hexpand(true)
                .hexpand_set(true)
                .halign(Align::Center)
                //.valign(Align::Center)
                .margin_top(40)
                .margin_bottom(40)
                //.width_chars(space)
                .build();
            new_grid.attach(&new_title_label, 1, 1, 1, 1);

            // Save built view
            // Create new view stack
            let new_stack: ViewStack = ViewStack::new();
            // Add object
            new_stack.add_titled(&new_grid, Some("default"), "Default");
            // Add icon
            //NOTE: see https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.ViewStack.html
            //      function "add_titled_with_icon" not in stable yet
            let new_item = new_stack.page(&new_grid);
            new_item.set_icon_name(Some("package-x-generic-symbolic"));
            // Replace current view stack
            self.imp().replace_stack(Some(&new_stack));
        } else {
            // Create temporary structure for sorting loaded data
            let mut loaded_views: Vec<String> = vec![String::from(""); loaded_views_data.len()];

            // Create new view stack
            let new_stack = ViewStack::new();

            // For each loaded view
            for index in 0..loaded_views_data.len() {
                // println!("item: `{}`", loaded_views_data[index]); //TEST

                // Split current item into the 4 parts
                let parts: Vec<&str> = loaded_views_data[index].split(':').collect::<Vec<&str>>();

                // Catch any malformed items
                if parts.len() != 3 {
                    panic!("Malformed gschema data..");
                }

                // If from valid page
                if parts[0] == self.property::<String>("uuid") {
                    // println!("VALID UUID"); //TEST

                    // If a valid position
                    match parts[1].parse::<usize>() {
                        Ok(position) => {
                            if position <= loaded_views_data.len() {
                                // println!("VALID POSITION INDEX: `{}`", position); //TEST

                                // Add to final list
                                loaded_views[position] = parts[2].to_owned();
                            }
                        }
                        Err(_) => panic!("Invalid Property position in gschema data.."),
                    }
                }
            }

            // Save final list of Views
            // println!("POPULATING VIEWS");
            let mut labels: Vec<Label> = Vec::new();
            let mut props: Vec<String> = Vec::new();
            for index in 0..loaded_views.len() {
                // println!("VIEW {}", index);
                // Grab all saved properties
                let properties: Vec<String> = self.check_properties_for_view(&loaded_views[index]);

                // Add prope
                for prop in &properties {rty items to the final list
                    props.push(prop.as_str().clone().to_owned());
                }

                // println!("GOT {} PROPERTIES FOR VIEW {}", properties.len(), index);

                // Create new view
                let new_grid_name: String = index.to_string() + "_grid";
                let new_grid: Grid = Grid::builder()
                    .name(&new_grid_name)
                    .orientation(Orientation::Horizontal)
                    //.margin_start(12)
                    //.margin_end(12)
                    //.margin_top(12)
                    //.margin_bottom(12)
                    //.halign(Align::Center)
                    //.valign(Align::Center)
                    //.hexpand(true)
                    //.vexpand(true)
                    .build();
                content_grid.attach(&new_grid, 0, 0 as i32, 100, 12);

                // Populate view given list of properties
                let output: (Grid, Vec<Label>) = self.create_properties(new_grid, properties, labels);
                let new_view_grid: Grid = output.0;
                labels = output.1;

                // Save built view
                // Add object
                let new_stack_item_name: String = index.to_string() + "_stack_item";
                new_stack.add_titled(&new_view_grid, Some(&new_stack_item_name), &loaded_views[index]);
                // println!("NEW STACK ITEM: `{}`", new_stack_item_name);//TEST
                // Add icon
                //NOTE: see https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.ViewStack.html
                //      function "add_titled_with_icon" not in stable yet
                let new_item = new_stack.page(&new_view_grid);
                new_item.set_icon_name(Some("package-x-generic-symbolic"));
            }

            self.create_updater(labels, props);

            // Replace current view stack
            self.imp().replace_stack(Some(&new_stack));
        }
    }

    /**
     * Name:
     * check_properties_for_view
     *
     * Description:
     * Using passed view title, returns list of saved properties
     *
     * Made:
     * 03/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn check_properties_for_view(&self, view_name: &str) -> Vec<String> {
        // Get reference to settings object
        let settings_obj: &Settings = self.settings();

        // Load list of Properties for current Page
        let loaded_properties_data: Vec<String> = settings_obj.get::<Vec<String>>("pageconfigs");
        // println!("items saved:`{}`", loaded_properties_data.len()); //TEST

        // If present in saved settings, use! otherwise follow below defaults
        if let 0 = loaded_properties_data.len() {
            match self
                .property::<Provider>("provider")
                .property::<i32>("provider-type")
            {
                -1 => vec![String::from("choose_provider")],
                _ => vec![String::from("choose_properties")],
            }
        } else {
            // Create temporary structure for sorting loaded data
            let mut loaded_properties: Vec<String> =
                vec![String::from(""); loaded_properties_data.len()];

            for index in 0..loaded_properties_data.len() {
                // println!("item: `{}`", loaded_properties_data[index]); //TEST

                // Split current item into the 4 parts
                let parts: Vec<&str> = loaded_properties_data[index]
                    .split(':')
                    .collect::<Vec<&str>>();

                // Catch any malformed items
                if parts.len() != 4 {
                    panic!("Malformed gschema data..");
                }

                // If from valid page
                if parts[0] == self.property::<String>("uuid") {
                    // println!("VALID UUID"); //TEST

                    // If from valid view
                    if parts[1] == view_name {
                        // println!("VALID VIEW #"); //TEST

                        // If a valid position
                        match parts[2].parse::<usize>() {
                            Ok(position) => {
                                if position <= loaded_properties_data.len() {
                                    // println!("VALID POSITION INDEX: `{}`", position); //TEST

                                    // println!("VALID PROPERTY: `{}`", parts[3]); //TEST

                                    // Add to final list
                                    loaded_properties[position] = parts[3].to_owned();
                                }
                            }
                            Err(_) => panic!("Invalid Property position in gschema data.."),
                        }
                    }
                }
            }

            // Remove any empty properties
            loaded_properties.retain(|x| *x != "");

            // Return final list
            loaded_properties.to_owned()
        }
    }

    /**
     * Name:
     * create_properties
     *
     * Description:
     * Add labels (of properties) to a passed grid, returns the grid afterwards
     *
     * Made:
     * 03/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn create_properties(&self, grid: Grid, properties: Vec<String>, mut labels: Vec<Label>) -> (Grid, Vec<Label>) {
        // Load properties from struct
        let properties_store: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(properties));

        // Grab grid manager
        let grid_manager = grid.layout_manager().unwrap();

        // For each Property
        for property in Arc::clone(&properties_store).lock().unwrap().iter() {
            // println!("BUILDING PROPERTY LABEL: `{}`", property); //TEST

            //==BUILD==
            // Build grid for 2 labels and attach to this page
            let new_grid_name: String = String::from("property_item_grid");
            let new_grid: Grid = Grid::builder()
                .name(&new_grid_name)
                .orientation(Orientation::Horizontal)
                //.margin_start(12)
                //.margin_end(12)
                //.margin_top(12)
                //.margin_bottom(12)
                //.halign(Align::Start)
                //.valign(Align::Center)
                //.hexpand(true)
                //.vexpand(true)
                .build();
            grid.attach(&new_grid, 0, labels.len() as i32, 100, 12);

            // Set layout properties of grid
            let child_manager: LayoutChild = grid_manager.layout_child(&new_grid);
            child_manager.set_property("row-span", 1);
            child_manager.set_property("column-span", 1);
            //child_manager.set_property("outline-style", "solid");
            //child_manager.set_property("outline-width", 1);
            //child_manager.set_property("border-radius", 3);

            // Fetch layout manager for this (grid) child
            match new_grid.layout_manager() {
                Some(internal_grid_manager) => {
                    // Decide on title label size
                    let space: i32;
                    let pretty_label: &str;
                    match property.to_owned().as_str() {
                        "util" => {
                            pretty_label = "GPU Utilization";
                            space = 5
                        }
                        "mem_ctrl_util" => {
                            pretty_label = "Memory Controller Utilization";
                            space = 5
                        }
                        "encoder_util" => {
                            pretty_label = "Encoder Utilization";
                            space = 5
                        }
                        "decoder_util" => {
                            pretty_label = "Decoder Utilization";
                            space = 5
                        }
                        "fan_speed" => {
                            pretty_label = "Fan Speed";
                            space = 5
                        }
                        "temp" => {
                            pretty_label = "Temperature";
                            space = 5
                        }
                        "memory_usage" => {
                            pretty_label = "Memory Usage";
                            space = 8
                        }
                        "memory_total" => {
                            pretty_label = "Memory Total";
                            space = 8
                        }
                        "power_usage" => {
                            pretty_label = "Power Usage";
                            space = 8
                        }
                        _ => {
                            pretty_label = property;
                            space = 5
                        }
                    }

                    // Build title label & add to grid
                    let new_title: String = String::from(property.to_owned()) + "_label";
                    let new_title_label: Label = Label::builder()
                        .label(pretty_label)
                        .name(&new_title)
                        .hexpand(true)
                        .hexpand_set(true)
                        //.halign(Align::Center)
                        .halign(Align::Start)
                        //.valign(Align::Center)
                        .margin_top(12)
                        .margin_bottom(12)
                        .width_chars(space)
                        .build();
                    new_grid.attach(&new_title_label, 0, 0, 1, 1);

                    // Set layout properties of (title label) child
                    let title_manager: LayoutChild =
                        internal_grid_manager.layout_child(&new_title_label);
                    title_manager.set_property("row-span", 1);
                    title_manager.set_property("column-span", 1);

                    // Decide on content label size
                    let space: i32;
                    match property.to_owned().as_str() {
                        "util" | "fan_speed" | "temp" => space = 5,
                        "memory_usage" | "memory_total" => space = 8,
                        _ => space = 5,
                    }

                    // Build content label & add to grid
                    let new_content: String = String::from(property.to_owned());
                    let new_content_label: Label = Label::builder()
                        .label("X")
                        .name(&new_content)
                        .halign(Align::End)
                        //.valign(Align::Center)
                        .width_chars(space)
                        .build();
                    new_grid.attach(&new_content_label, 1, 0, 1, 1);

                    // Set layout properties of (content label) child
                    let content_manager: LayoutChild =
                        internal_grid_manager.layout_child(&new_content_label);
                    content_manager.set_property("row-span", 1);
                    title_manager.set_property("column-span", 1);

                    // Add to list of content labels, for updating in closure (see below)
                    labels.push(new_content_label);
                }
                None => panic!("Cannot find layout manager.."),
            }
        }

        // for lab in &labels {
        //     println!("LABEL: `{}`", lab.widget_name());
        // }

        // Return modified grid object
        (grid, labels)
    }

    /*
     * Name:
     * create_updater
     *
     * Description:
     * Creates a recurring closure to fill the passed list of labels
     * Stores the ID of the recurring closure to allow removal
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * <https://stackoverflow.com/questions/70986004/gtk-rs-set-label-within-glibtimeout-add>
     * <https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html>
     *
     * !!UNSAFE CODE HERE!!
     */
    fn create_updater(&self, labels: Vec<Label>, properties: Vec<String>) {
        // !!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!
        // Check for any running recurring closures
        // Grab stored ID
        let id_raw: u32 = self.property("refreshid");
        unsafe {
            // If the stored ID is valid
            if id_raw != 0 {
                // Re-translate to SourceId object
                let id: SourceId = FromGlib::from_glib(id_raw);

                // Remove recurring closure
                id.remove();

                // println!("REMOVED RECURRING CLOSURE.."); //TEST
            } else {
                // println!("NO PRE-EXISTING RECURRING CLOSURE.."); //TEST
            }
        }
        // !!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!

        // Get reference to settings object
        let settings_obj: &Settings = self.settings();

        // Load refresh time (s) from settings
        let refresh_rate: u32 = settings_obj.get::<i32>("refreshrate") as u32;


        // Create thread safe container for properties
        let properties_store: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(properties));

        // Create thread safe container for labels
        let label_store: Arc<Mutex<Vec<Label>>> = Arc::new(Mutex::new(labels));

        // Create thread safe container for uuid, needed for processor
        let uuid_store: Arc<Mutex<String>> = Arc::new(Mutex::new(self.property("uuid")));

        // Create thread safe container for provider
        let provider_store: Arc<Mutex<Option<Provider>>> =
            Arc::new(Mutex::new(self.property("provider")));

        // Async fill the labels
        let id: SourceId = glib::timeout_add_seconds_local(refresh_rate, move || {
            // Grab locked data
            // list of Properties
            let properties_lock: Arc<Mutex<Vec<String>>> = Arc::clone(&properties_store);
            let properties: MutexGuard<Vec<String>> = properties_lock.lock().unwrap();
            // uuid
            let uuid_lock: Arc<Mutex<String>> = Arc::clone(&uuid_store);
            let uuid: String = uuid_lock.lock().unwrap().as_str().to_owned();
            // current provider for scanning gpu data
            let provider_lock: Arc<Mutex<Option<Provider>>> = Arc::clone(&provider_store);
            let mut provider_container: MutexGuard<Option<Provider>> =
                provider_lock.lock().unwrap();

            let labels_lock: Arc<Mutex<Vec<Label>>> = Arc::clone(&label_store);
            let labels_container: MutexGuard<Vec<Label>> = labels_lock.lock().unwrap();
            // println!("labels: `{}`", labels_container.len());

            // For each Property
            match &mut *provider_container {
                Some(current_provider) => {
                    for property in properties.iter() {
                        // Grab current Property from provider
                        match current_provider.get_gpu_data(&uuid, property) {
                            Ok(property_value) => {
                                // For each output label of the page
                                for label in labels_container.iter() {
                                    // println!("COMPARING AGAINST 1: `{}`", property);
                                    // println!("COMPARING AGAINST 2: `{}`", label.widget_name());

                                    // Check if correct label
                                    if *property.to_owned() == label.widget_name() {
                                        label.set_label(&property_value);
                                    }
                                }
                            }
                            Err(err) => {
                                println!("panicked when fetching gpu data: `{}`", err);
                                return Continue(false);
                            }
                        }
                    }
                }
                None => {
                    println!("panicked when fetching current provider..");
                    return Continue(false);
                }
            }

            Continue(true)
        });

        // !!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!
        // Save ID of recurring closure
        unsafe {
            self.set_property("refreshid", id.as_raw());
        }
        // !!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!
    }

    /**
     * Name:
     * setup_widgets
     *
     * Description:
     * Set up all widgets
     *
     * Made:
     * 23/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn setup_widgets(&self) {
        self.load_views();
    }
}

/**
 * Trait Name:
 * Default
 *
 * Description:
 * Default object
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
impl Default for GpuPage {
    fn default() -> Self {
        Self::new("", "Default", Provider::default())
    }
}
