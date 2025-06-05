
use fltk::{
    button::RadioButton,
    group::Group,
    prelude::*,
    window::Window,
};

/// Attempt to create  vrbl_create()  using checkbox & radio buttons.

pub fn vrbl_create() {
    let mut var1 = Variable::new();
    vrbl_choose_type(&var1);
}

pub fn vrbl_choose_type() -> String {
    /// region Create a window and group to hold the radio buttons.
    let mut wind = Window::new(100, 100, 200, 200, "Choose Variable Type");
    let mut group = Group::new(10, 10, 180, 180, None);

    // Create radio buttons
    let mut string_btn = RadioButton::new(20, 30, 140, 25, "String");
    let mut char_btn = RadioButton::new(20, 65, 140, 25, "Character");
    let mut int_btn = RadioButton::new(20, 100, 140, 25, "Integer");
    let mut decimal_btn = RadioButton::new(20, 135, 140, 25, "Decimal");

    group.end();
    wind.end();
    wind.set_modal(true);
    wind.show();
    //endregion

    // region Add the radio buttons to the group

    // Set the first option as default
    string_btn.set(true);

    // Track the selected button
    let mut selected = String::from("String");

    while wind.shown() {   // todo: This seems clusy.  Would a match be better?
        fltk::app::wait();

        if string_btn.is_toggled() {
            selected = String::from("String");
        } else if char_btn.is_toggled() {
            selected = String::from("Character");
        } else if int_btn.is_toggled() {
            selected = String::from("Integer");
        } else if decimal_btn.is_toggled() {
            selected = String::from("Decimal");
        }
    }
    //endregion

    selected
}

    
    