
//! # Question Bank Creator
//! This program is targeted at teachers & homeschool parents and is
//! especially useful for teaching math and science,
//! although it also can be useful as an aide in the teaching of
//! other disciplines.
//! Curriculum developers will especially find it useful.  It allows
//! the construction of test/worksheet/quiz/individual practice questions
//! that contain dynamic content.  It then saves those questions -- using
//! the json markdown language -- in user-defined ‘question banks’,
//! thus keeping related questions together in the same file.
//! A parent or teacher can create variables that generate dynamic values
//! (either numeric, character, or string) using random or pseudo-random
//! criteria set by the user.  Once constructed, the question is stored
//! in a file (or 'question bank') for later access as needed.
//! Parents or teachers can also make the app/questions available to
//! students for student-directed practice.
//!
//!
//!    * VERSION = "0.29.7";
//!    * AUTHOR = "John T. Reagan";
//!    * LICENSE = "GNU Version 3";
//!    * LICENSE_URL = "<https://opensource.org/license/agpl-v3>";
//!    * COPYRIGHT = "Copyright (c) 2025, John T. Reagan";
//!    * REPOSITORY = "<https://github.com/jtreagan/Question-Bank-Creator>";


/*                        Thoughts & Ideas

    -- It does make sense to save the variables as files so that
            they may be reused globaly across different questions and banks.

    -- Eventually you will want to enter the objectives and prerequistes through
            a menu that lists all available items.

    // TODO: Add a menu item for creating and adding objectives and
            prerequisites.

    -- Note that both o's and p's sometimes have images associated with them.
            How are you going to handle that?
    -- Do you want to attach p's to associated o's?
        -- Yes.

    //TODO: Provide for re-ordering/moving questions in a bank.

 */  // Thoughts & Ideas

use fltk::{app::*, prelude::*, prelude::WidgetExt};
use rebuild6::{menus::*, misc::*};
use rebuild6::{APP_FLTK, WIDGETS};
use rebuild6::{Wdgts};

fn main() {
    let app = App::default();
    {
        *APP_FLTK.lock().unwrap() = app.clone();  
    }  // Store the app in the global variable.

    set_font_size(20);
    let mut wdgts = Wdgts::new();
    let menubar = qbnk_menubar(&mut wdgts.prim_win);
    wdgts.prim_win.add(&menubar);
    primwin_setup(&mut wdgts.prim_win);

    {
        *WIDGETS.lock().unwrap() = wdgts.clone();
    }  // Store wdgts in the global variable.

    wdgts.prim_win.show();

    /*
    //region Set up and call the on-open popup window.
        // todo:  Why can't I move the popup stuff below to a separate function?
        //          It doesn't work.  I tried it.

                // region Set up button callback closures.
    // Button -- Create a new question bank.
    let bttn_newbank = move || {
        bnk_create();
        bnk_refresh_widgets();
    };

    // Button -- Open an existing question bank.
    let bttn_openbank = move || {
        bnk_read();
        bnk_refresh_widgets();
    };
                // endregion

    let mut pop = fltk_popup_2btn(&wdgts.prim_win, Box::new(bttn_newbank), "Create new bank",
                                  Box::new(bttn_openbank), "Open existing bank");
    pop.set_color(Color::Red);

    pop.end();
    pop.show();
    // endregion
    */  // Deletemeplease

    app.run().unwrap();
}

