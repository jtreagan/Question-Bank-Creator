
/*                        Thoughts & Ideas

    -- It does make sense to save the variables as files so that
            they may be reused globaly across different questions and banks.

    // TODO: Merge   lib_fltk   &   lib_input_fltk   libraries.

    -- Bank title display -- It would be nice if the title display at the top
            of the display screen was also editable, but when I tried to do
            that with FLTK's TextEditor struct it doesn't have that functionality.
            I even got that confirmed by  MoAloysef  on the  fltk.rs github
            forum.  This is a big weakness.  Makes me wonder if a different GUI might
            be better.

    -- Eventually you will want to enter the objectives and prerequistes through
            a menu that lists all available items.

    // TODO: Add a menu item for creating and adding objectives and
            prerequisites.

    -- Note that both o's and p's sometimes have images associated with them.
            How are you going to handle that?
    -- Do you want to attach p's to associated o's?

    //TODO: Provide for re-ordering/moving questions in a bank.

 */  // Thoughts & Ideas

// region Credits Constants

const _AUTHOR: &str = "John T. Reagan";
const _DESCRIPTION: &str = "Create and edit question banks with dynamic content.  \n";
const _LICENSE: &str = "MIT";
const _COPYRIGHT: &str = "Copyright (c) 2021 <John T. Reagan>";

// endregion

use fltk::{app::*, prelude::*, prelude::WidgetExt, window::Window, text, window};
use fltk::button::Button;
use fltk::enums::{Color};
use fltk::group::Scroll;
use fltk::text::{TextBuffer, TextDisplay, TextEditor};
use lib_myfltk::fltkutils::fltk_popup_2btn;
use qbnk_rbld5::{menus::*, misc::*};
use qbnk_rbld5::{APP_FLTK, CURRENT_BANK, DEVELOPMENT_VERSION, PROGRAM_TITLE, QDISP_HEIGHT, VERSION, WIDGETS};
use qbnk_rbld5::{banks::*, Wdgts, questions::qst_edit};

fn main() {
    let app = App::default();
    *APP_FLTK.lock().unwrap() = app.clone();  // Store the app in the global variable.

    set_font_size(20);
    let mut wdgts = Wdgts::new();
    let menubar = qbnk_menubar(&mut wdgts.prim_win);
    wdgts.prim_win.add(&menubar);
    primwin_setup(&mut wdgts.prim_win);

    *WIDGETS.lock().unwrap() = wdgts.clone();

    wdgts.prim_win.show();

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

    app.run().unwrap();
}

