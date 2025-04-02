
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

use fltk::{app::*, prelude::*, prelude::WidgetExt, window::Window, text};
use fltk::button::Button;
use fltk::enums::{Color};
use fltk::group::Scroll;
use fltk::text::{TextBuffer, TextDisplay, TextEditor};
use lib_myfltk::fltkutils::fltk_popup_2btn;
use qbnk_rbld5::menus::*;
use qbnk_rbld5::{APP_FLTK, CURRENT_BANK, DEVELOPMENT_VERSION, PROGRAM_TITLE, QDISP_HEIGHT, VERSION, WIDGETS};
use qbnk_rbld5::{banks::*, Wdgts, questions::qst_edit};

fn main() {
    let app = App::default();
    *APP_FLTK.lock().unwrap() = app.clone();  // Store the app in the global variable.

    let mut primwin = primwin_setup();
    let menubar = qbnk_menubar(&mut primwin);            // menubar is NOT stored in the widget struct.
    primwin.add(&menubar);

    make_title_txtedtr(&mut primwin);                                         // Add a title TextEditor.
    make_scrollgroup(&mut primwin);                                           // Add a scrollbar.

    primwin.end();
    primwin.show();

    let mut initpopup = onopen_popup(&primwin);                                        // Ensures that a bank is loaded.
    initpopup.set_color(Color::Red);
    initpopup.end();
    initpopup.show();

    app.run().unwrap();
}

pub fn primwin_setup() -> Window {  // Set up the primary window.
    let mut primwin = Window::default().with_size(825, 900).with_pos(1000, 100);
    set_font_size(20);
    primwin.set_color(Color::Blue);
    let fulltitle = format!("{} -- {} -- Version {}", DEVELOPMENT_VERSION, PROGRAM_TITLE, VERSION);
    primwin.set_label(fulltitle.as_str());
    primwin.make_resizable(true);

    primwin
}

pub fn onopen_popup(primwin: &Window) -> Window {
    // On program opening, pop up a window with choice for new bank or open existing bank.

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

    // todo: The function below is way too complex.  Find another solution.
    let pop = fltk_popup_2btn(&primwin, Box::new(bttn_newbank), "Create new bank",
                    Box::new(bttn_openbank), "Open existing bank");
    pop
}

pub fn make_title_txtedtr(primwin: &mut Window) {

    // todo: Add a line, smaller font, below the main title for
    //      a subtitle or maybe the title of the textbook being used.

    let usebank: Bank;
    let mut wdgts: Wdgts;
    {
        usebank = CURRENT_BANK.lock().unwrap().clone();
        wdgts = WIDGETS.lock().unwrap().clone();
    }  // Load the global structs.

    let mut buf = TextBuffer::default();
    buf.set_text(usebank.bank_title.as_str());  // Uses the title from the current bank.

    let mut ted = TextEditor::new(0, 40, primwin.width(), 60, "");
    ted.set_text_size(32);
    ted.set_text_color(fltk::enums::Color::White);
    ted.set_color(fltk::enums::Color::DarkMagenta);
    ted.set_buffer(buf.clone());   // Clone is used here to avoid an ownership error.

    wdgts.title_editbox = ted.clone();  // Store the widgit in the widget struct.
    primwin.add(&ted.clone());

    *WIDGETS.lock().unwrap() = wdgts.clone();    // Update the WIDGET global variable.

    // todo: It would be nice to center and bold the text, but that is really
    //      difficult to do, so leave for later.
}

pub fn make_scrollgroup(primwin: &mut Window) -> Scroll {

    let mut wdgts: Wdgts;
    {
        wdgts = WIDGETS.lock().unwrap().clone();
    }  // Access the struct containing the primary window's widgets.

    // Create scroll group
    let mut scroll = Scroll::new(0, wdgts.title_editbox.height() + 1,
                                 primwin.width(),
                                 primwin.height() - wdgts.title_editbox.height(),
                                 "");
    scroll.set_scrollbar_size(15);

    // Add scroll to the Wdgts struct & window.
    wdgts.scroll = scroll.clone();
    //primwin.add(&scroll.clone());

    *WIDGETS.lock().unwrap() = wdgts.clone();    // Update the WIDGET global variable.

    scroll
}

pub fn make_question_boxes(primwin: &mut Window) {
    let usebank: Bank;
    let mut wdgts: Wdgts;
    {
        usebank = CURRENT_BANK.lock().unwrap().clone();
        wdgts = WIDGETS.lock().unwrap().clone();
    }  // Load the global structs.

    //Create and add TextDisplay boxes and buttons to the widget struct.

    let mut box_y = wdgts.title_editbox.height() + 1;  // Allow room for the Title Box
    let mut qnum = 1;  // Question number -- starts at 1.

        // The loop below sets up display boxes for each question in the bank.
    for item in usebank.question_vec.iter() {

        // region Create the question label and set up text buffer.
        let qlabel = format!("Question {} :  ", qnum);
        let mut txtbuff = TextBuffer::default();
        txtbuff.set_text(item.qtext.as_str());
        // endregion

        // region Setup the display box and it's attributes.
        let mut qdisp = TextDisplay::new(0, box_y, primwin.width(), QDISP_HEIGHT, qlabel.as_str());
        qdisp.set_buffer(txtbuff);
        qdisp.wrap_mode(text::WrapMode::AtBounds, 0);
        qdisp.set_color(fltk::enums::Color::White);
        qdisp.set_text_size(22);
        qdisp.set_text_color(fltk::enums::Color::Black);
        // endregion

        // region Setup the edit button & callback. Buttons not added to widget struct.
        let editbtn_x = qdisp.x() + qdisp.w() - 50;  // Button is sized 50 X 30
        let editbtn_y = qdisp.y() + qdisp.h() - 30;
        let mut editbtn = Button::new(editbtn_x, editbtn_y, 50, 30, "Edit");

        editbtn.set_callback(move |_| {
            println!("\n Edit button for Question #{} has been pressed. \n", qnum);
            qst_edit(qnum - 1);
        });
        // endregion

        // TODO: Set up show/edit prereqs and objectives button
        // TODO: Create a subframe to display/edit the answer.

        box_y += QDISP_HEIGHT;    // Increment the question display widget position.
        qnum += 1;       // Increment the question display number.

        wdgts.qstn_boxes.push(qdisp.clone());
        wdgts.scroll.add(&qdisp);
    }
    *WIDGETS.lock().unwrap() = wdgts.clone();    // Update the WIDGET global variable.
}