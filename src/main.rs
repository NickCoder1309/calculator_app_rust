use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, CssProvider, Entry, Grid, gdk};

fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);
    window.set_title(Some("Calculator"));
    window.set_default_size(300, 450);

    //Create an entry for user input
    let main_entry = Entry::new();
    main_entry.set_enable_undo(true);
    main_entry.set_margin_bottom(3);
    main_entry.set_margin_top(3);
    main_entry.set_margin_start(3);
    main_entry.set_margin_end(3);

    //Create grid to place the calculator buttons
    let main_grid = Grid::builder()
        .hexpand_set(true)
        .vexpand_set(true)
        .row_spacing(1)
        .column_spacing(1)
        .build();

    //Add grid to the window
    window.set_child(Some(&main_grid));

    //Vector for storing calculator buttons
    let mut numbers_vector: Vec<&mut Button> = Vec::new();

    //Vector for storing operator buttons
    let mut operators_vector: Vec<&mut Button> = Vec::new();

    //Vector for storing extras buttons
    let mut extras_vector: Vec<&mut Button> = Vec::new();

    let mut button_0 = Button::builder().label("0").build();
    numbers_vector.push(&mut button_0);

    let mut button_1 = Button::builder().label("1").build();
    numbers_vector.push(&mut button_1);

    let mut button_2 = Button::builder().label("2").build();
    numbers_vector.push(&mut button_2);

    let mut button_3 = Button::builder().label("3").build();
    numbers_vector.push(&mut button_3);

    let mut button_4 = Button::builder().label("4").build();
    numbers_vector.push(&mut button_4);

    let mut button_5 = Button::builder().label("5").build();
    numbers_vector.push(&mut button_5);

    let mut button_6 = Button::builder().label("6").build();
    numbers_vector.push(&mut button_6);

    let mut button_7 = Button::builder().label("7").build();
    numbers_vector.push(&mut button_7);

    let mut button_8 = Button::builder().label("8").build();
    numbers_vector.push(&mut button_8);

    let mut button_9 = Button::builder().label("9").build();
    numbers_vector.push(&mut button_9);

    let mut erase_one_button = Button::builder().label("<-").build();
    extras_vector.push(&mut erase_one_button);

    let mut clean_button = Button::builder().label("AC").build();
    extras_vector.push(&mut clean_button);

    let mut percentage_button = Button::builder().label("%").build();
    extras_vector.push(&mut percentage_button);

    let mut division_button = Button::builder().label("/").build();
    operators_vector.push(&mut division_button);

    let mut multiply_button = Button::builder().label("X").build();
    operators_vector.push(&mut multiply_button);

    let mut substract_button = Button::builder().label("-").build();
    operators_vector.push(&mut substract_button);

    let mut addition_button = Button::builder().label("+").build();
    operators_vector.push(&mut addition_button);

    let mut equal_button = Button::builder().label("=").build();
    operators_vector.push(&mut equal_button);

    //Set buttons styles
    set_buttons_style(numbers_vector, 2, 2, 2, 2, "round-btn-numbers");
    set_buttons_style(operators_vector, 2, 2, 2, 2, "round-btn-operators");
    set_buttons_style(extras_vector, 2, 2, 2, 2, "round-btn-extras");

    //Attach main entry to the main grid
    main_grid.set_hexpand(true);
    main_grid.set_vexpand(true);
    main_grid.attach(&main_entry, 0, 0, 4, 1);

    //Assign buttons to especific position in the grid
    main_grid.attach(&button_1, 0, 4, 1, 1);
    main_grid.attach(&button_2, 1, 4, 1, 1);
    main_grid.attach(&button_3, 2, 4, 1, 1);
    main_grid.attach(&button_4, 0, 3, 1, 1);
    main_grid.attach(&button_5, 1, 3, 1, 1);
    main_grid.attach(&button_6, 2, 3, 1, 1);
    main_grid.attach(&button_7, 0, 2, 1, 1);
    main_grid.attach(&button_8, 1, 2, 1, 1);
    main_grid.attach(&button_9, 2, 2, 1, 1);
    main_grid.attach(&button_0, 1, 5, 1, 1);
    main_grid.attach(&erase_one_button, 0, 1, 1, 1);
    main_grid.attach(&clean_button, 1, 1, 1, 1);
    main_grid.attach(&percentage_button, 2, 1, 1, 1);
    main_grid.attach(&division_button, 3, 1, 1, 1);
    main_grid.attach(&multiply_button, 3, 2, 1, 1);
    main_grid.attach(&substract_button, 3, 3, 1, 1);
    main_grid.attach(&addition_button, 3, 4, 1, 1);
    main_grid.attach(&equal_button, 3, 5, 1, 1);

    window.present();
}

fn main() {
    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(|app| {
        let css_numbers_provider = CssProvider::new();
        css_numbers_provider.load_from_string(
            "
            .round-btn-numbers {
                background: #696969;
                color: white;
                border-radius: 50%;
                min-width: 80 px;
                min-height: 80 px;
                font-weight: bold;
                border: none;
            }

            .round-btn-numbers:hover {
                background: #000000;
            }
            ",
        );

        let css_operators_provider = CssProvider::new();
        css_operators_provider.load_from_string(
            "
            .round-btn-operators {
                background: #FFA500;
                color: white;
                border-radius: 50%;
                min-width: 80 px;
                min-height: 80 px;
                font-weight: bold;
                border: none;
            }

            .round-btn-operators:hover {
                background: #FF8C00;
            }
            ",
        );

        let css_extras_provider = CssProvider::new();
        css_extras_provider.load_from_string(
            "
            .round-btn-extras {
                background: #DCDCDC;
                color: white;
                border-radius: 50%;
                min-width: 80 px;
                min-height: 80 px;
                font-weight: bold;
                border: none;
            }

            .round-btn-extras:hover {
                background: #778899;
            }
            ",
        );

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display"),
            &css_operators_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display"),
            &css_numbers_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display"),
            &css_extras_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        on_activate(app);
    });
    // Run the application
    app.run();
}

fn set_buttons_style(
    buttons: Vec<&mut Button>,
    margin_botton: i32,
    margin_top: i32,
    margin_start: i32,
    margin_end: i32,
    css_class: &str,
) {
    for element in buttons {
        element.set_margin_bottom(margin_botton);
        element.set_margin_top(margin_top);
        element.set_margin_start(margin_start);
        element.set_margin_end(margin_end);
        element.set_hexpand(true);
        element.set_vexpand(true);
        element.add_css_class(css_class);
    }
}
