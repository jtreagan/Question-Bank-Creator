
//! # Question Bank Creator
//! This program is targeted at teachers & homeschool parents and is
//! especially useful for teaching math and science,
//! although it also can be useful as an aide in the teaching of
//! other disciplines.
//! Curriculum developers will especially find it useful.  It allows
//! the construction of test/worksheet/quiz/individual practice questions
//! that contain dynamic content.  It then saves those questions -- using
//! serde.json -- in user-defined ‘question banks’,
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


/*
       // TODO: Font Libre Baskerville, 14 pt.  -- next iteration.
       // TODO: All this needs to be user modifiable. -- next iteration.
       // TODO: Goal is for  WYSIWIG.  --  in the far, far future.

       // TODO: Add second line to the title containing the associated textbook text.
       // TODO: Question display should show calculated values for the variables
       //          rather than the variable ID.  Maybe highlight the values so
       //          that the variable can be easily located.  May also want to set up
       //          a hover over the value/variable to show the variable's value/variable name.

       // TODO: Refine the implementation of how you use LastDirUsed.  It is currently
       //          inconsistent in how it is applied.  Check every instance where a
       //          file is read or written -- primarily in the *_read() and *_save()
       //          functions.  Also, check the three utility crates.  The
       //          file_browse_save_fltr() function will need attention.

       // TODO:  In all the *_read() functions, the `if` statement that checks to make sure
       //            the file being read is not empty could possibly be turned into a function.
       //            This would make the code more readable and easier to maintain.

*/
// TODO's

use crate::banks::Bank;
use fltk::app::App;
use fltk::group::Scroll;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::text::{TextDisplay, TextEditor};
use fltk::utils::oncelock::Lazy;
use fltk::window;
use fltk::window::Window;
use std::sync::Mutex;

// region  Global Constants

/// The current iteration of the program being worked on.
///
pub const DEVELOPMENT_VERSION: &str = "Question Bank Rebuild 4";
/// The title of the project.
///
pub const PROGRAM_TITLE: &str = "Question Bank Creator";
/// The current version..
///
pub const VERSION: &str = "0.29.8"; // Note:  Versioning, while semantic in format, is decimal in nature.

// Note:  The constants below are here during development.  They will be moved to
//          a config file once the program is more stable.

/// The default folder where data is saved.
///
pub const DATA_GENERAL_FOLDER: &str = "/home/jtreagan/programming/mine/qbnk_rb7/src/qbnk_data";
/// The default folder for saving Lists.
///
pub const LIST_DIR: &str = "/home/jtreagan/programming/mine/qbnk_rb7/src/qbnk_data/lists";
/// The default folder for saving Variables.
///
pub const VARIABLE_DIR: &str = "/home/jtreagan/programming/mine/qbnk_rb7/src/qbnk_data/variables";
/// The default folder for saving Banks.
///
pub const BANK_DIR: &str = "/home/jtreagan/programming/mine/qbnk_rb7/src/qbnk_data/banks";

/// Default height of the question display.
///
pub const QDISP_HEIGHT: i32 = 150;
/// Default width of the scrollbar group.
///
pub const SCROLLBAR_WIDTH: i32 = 15;
// endregion

//region Global Variables
/// Contains the question Bank that is currently being edited.
///
pub static CURRENT_BANK: Lazy<Mutex<Bank>> = Lazy::new(|| Mutex::new(Bank::new()));
/// Contains the last directory path that was used.
///
pub static LAST_DIR_USED: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
/// Holds the currently running FLTK App.
///
pub static APP_FLTK: Lazy<Mutex<App>> = Lazy::new(|| Mutex::new(App::default()));
/// Holds the FLTK widgets currently being used.
///
pub static WIDGETS: Lazy<Mutex<Wdgts>> = Lazy::new(|| Mutex::new(Wdgts::new()));
//endregion

// region Global Structs & Enums
/// Struct that holds the primary window's widgets.
///
pub struct Wdgts {
    pub prim_win: Window,
    pub title_editbox: TextEditor,
    pub scroll: Scroll,
    pub qstn_boxes: Vec<TextDisplay>,
}

impl Default for Wdgts {
    fn default() -> Self {
        let prim_win = window::Window::new(1100, 200, 825, 900, PROGRAM_TITLE)
            .with_size(825, 900)
            .with_pos(1100, 200);
        prim_win.end();

        Self {
            prim_win,
            title_editbox: TextEditor::default(),
            scroll: Scroll::default(),
            qstn_boxes: Vec::new(),
        }
    }
}

impl Wdgts {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Clone for Wdgts {
    fn clone(&self) -> Self {
        Self {
            prim_win: self.prim_win.clone(),
            title_editbox: self.title_editbox.clone(),
            scroll: self.scroll.clone(),
            qstn_boxes: self.qstn_boxes.clone(),
        }
    }
}

// endregioncar

/// Holds the TypeWrapper enum.
///
pub mod global {

    use crate::{BANK_DIR, DATA_GENERAL_FOLDER, LAST_DIR_USED};
    use lib_file::dir_mngmnt::dir_normalize_path;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum TypeWrapper {
        Alphanum(String),
        Letter(char),
        Integer(i64),
        Floating(f64),
    }

    /// This function retrieves the last directory used and ensures it is
    ///     normalized as a proper directory path.
    ///
    /// If the `LAST_DIR_USED` global variable is empty, it sets it to
    /// the default directory specified by `BANK_DIR`. It then normalizes
    /// the path of the directory to ensure it ends in a valid directory
    /// format (and not a file).
    ///
    /// # Returns
    ///
    /// * A `String` representing the normalized directory path.
    ///
    /// # Behavior
    ///
    /// - If `LAST_DIR_USED` is empty, it initializes it to the `BANK_DIR` value, a
    ///     global constant that specifies the default directory for question banks.
    /// - Ensures the directory path is properly normalized using the helper
    ///     function `dir_normalize_path`.
    /// - Makes use of a global variable `LAST_DIR_USED` that is protected by
    ///     a `Mutex` for thread-safe operations.
    ///
    /// # Panics
    ///
    /// This function will panic if the `Mutex` lock on `LAST_DIR_USED` cannot be obtained,
    /// or if there is a logic failure when attempting to normalize the directory path.
    ///
    /// # Dependencies
    ///
    /// This function uses the directory management functions in the `lib_file` crate.
    ///
    /// # Example
    ///
    ///     use std::sync::Mutex;
    ///     use fltk::utils::oncelock::Lazy;
    ///     use lib_file::dir_mngmnt::dir_normalize_path;
    ///
    ///     pub const BANK_DIR: &str = "/home/jtreagan/programming/mine/qbnk_rb7/src/qbnk_data/banks";
    ///     pub static LAST_DIR_USED: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
    ///
    ///     fn main() {
    ///         *LAST_DIR_USED.lock().unwrap() = "/home/jtreagan/programming/mine/deleteme3/somefile.fde".to_string().clone();
    ///         let path = glob_check_lastdirused();
    ///         println!("\n {:?} \n", path);
    ///     }
    ///
    ///
    pub fn glob_check_lastdirused() -> String {
        if LAST_DIR_USED.lock().unwrap().clone() == "" {  // Set to default directory.
            *LAST_DIR_USED.lock().unwrap() = DATA_GENERAL_FOLDER.to_string().clone();
        }
        let usedir = LAST_DIR_USED.lock().unwrap().clone();

        dir_normalize_path(&usedir)  // Makes sure the path ends on a folder, not a file.
    }



} // End   global   module

/// Functions that deal with the Bank struct.
///
pub mod banks {
    use std::fs::File;
    use std::io::Write;
    use crate::misc::{dir_is_empty, make_question_boxes, make_scrollgroup, make_title_txtedtr};
    use crate::{questions::*, Wdgts, APP_FLTK, BANK_DIR, CURRENT_BANK, LAST_DIR_USED, WIDGETS};
    use fltk::dialog::{choice2_default, message_title};
    use fltk::prelude::{DisplayExt, GroupExt, WidgetExt};
    use fltk::text::TextBuffer;
    use lib_file::{dir_mngmnt::dir_normalize_path, file_fltk::*};
    use lib_file::file_mngmnt::file_read_to_string;
    use lib_myfltk::fltkutils::fltk_custom_message;
    use lib_myfltk::input_fltk::*;
    use serde::{Deserialize, Serialize};
    use crate::global::glob_check_lastdirused;
    //region Struct Section

    /// The outermost of the three structs QBC is built around.
    /// Note that the field `question_vec: Vec<Question>` is a vector
    /// of type `Question`.  `Question` is the second layer in the
    /// three-struct nest.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Bank {
        pub bank_title: String,          // Also used for file name.
        pub associated_textbook: String, // Use  ""  if no text being used.
        pub question_vec: Vec<Question>,
    }

    impl Default for Bank {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Bank {
        /// Initialize a new question Bank.
        pub fn new() -> Bank {
            Self {
                bank_title: "No Bank Loaded".to_string(),
                associated_textbook: "Untitled Textbook".to_string(),
                question_vec: Vec::new(),
            }
        }
    }

    impl Clone for Bank {
        /// Clone a question Bank.
        fn clone(&self) -> Self {
            Self {
                bank_title: self.bank_title.clone(),
                associated_textbook: self.associated_textbook.clone(),
                question_vec: self.question_vec.clone(), // Vec itself does implement Clone.
            }
        }
    }

    //endregion


    /// Checks if a bank is currently loaded.
    ///
    /// This function determines whether a bank is loaded in memory by checking
    /// the `CURRENT_BANK` global variable. If the `bank_title` of the current bank
    /// is anything other than "No Bank Loaded", the function returns `true`, indicating
    /// that a bank is loaded. Otherwise, it returns `false`.
    ///
    /// # Returns
    ///
    /// * `true` - if a bank is loaded.
    /// * `false` - if no bank is currently loaded.
    ///
    /// # Example
    /// main() {
    ///     if bnk_check_loaded() {
    ///         println!("A bank is loaded in memory.");
    ///     } else {
    ///         println!("No bank is loaded.");
    ///     }
    /// }
    /// # Note
    ///
    /// This function involves locking the `CURRENT_BANK` mutex for thread-safe access.
    /// Ensure that this does not lead to any potential deadlocks in your program.
    pub fn bnk_loaded() -> bool {
        let mut inmem = false;
        {
            if CURRENT_BANK.lock().unwrap().clone().bank_title != "No Bank Loaded" {
                inmem = true;
            }
        }
        inmem
    }

    /// Refreshes the contents of the widgets that are currently being displayed
    /// and edited in the primary window.
    pub fn bnk_refresh_widgets() {
        let mut wdgts: Wdgts;
        {
            wdgts = WIDGETS.lock().unwrap().clone();
        }

        wdgts.scroll.clear();
        wdgts.scroll.redraw();

        // Create/refresh widgets based on data in CURRENT_BANK.
        make_title_txtedtr();
        make_scrollgroup();
        make_question_boxes();
    }

    /// Creates a brand new question bank.
    ///
    pub fn bnk_create() {
        // todo: Check for bank in memory before proceeding.

        let app;
        {
            app = *APP_FLTK.lock().unwrap();
        } // Access the main app.

        // Input values into the struct fields.
        let mut newbank = Bank::new();
        newbank.bank_title = input_string(&app, "Please enter the bank's title.", 300, 90);
        newbank.associated_textbook = input_string(
            &app,
            "If you are using an associated textbook \
         please enter its info. \n Press  Enter  if no textbook is being used.",
            800,
            200,
        );

        // Pass the new bank into CURRENT_BANK
        {
            *CURRENT_BANK.lock().unwrap() = newbank.clone();
        }

        // Save and display the bank.
        bnk_save();

        // Create widgets based on the data in CURRENT_BANK.
        //make_title_txtedtr();
        //make_scrollgroup();
        //make_question_boxes();
    }

    /// Reads a question bank's data from a file.
    ///
    pub fn bnk_read() {

        // region Check if a bank is already loaded.  If so, ask the user if they want to replace it.
        if bnk_loaded() {
            message_title("A bank is already in memory.");  // todo: If bank is loaded section still needs help.
            let choice = choice2_default("Save and replace the existing bank in memory?", "Yes", "No", "");
            match choice {
                Some(0) => {  // User chose "Yes".
                    bnk_save();
                }
                Some(1) => {  // User chose "No".  Do nothing and return.
                    message_title("No bank loaded");
                    return;
                }
                _ => {  // Some unexpected value was chosen.  Do nothing and return.
                    message_title("No bank loaded");
                    return;
                }
            }
        }
        // endregion

        // region Set up directories.

        let usedir = glob_check_lastdirused();  // Handles the case where LAST_DIR_USED is empty.
        let readpath: String;  // This will be the path to the file that is to be read.

        if !dir_is_empty(&usedir) {
            readpath = file_fullpath(&usedir, "Choose the Bank file you want to read.");
        } else {
            fltk_custom_message("The directory you chose is empty.","Return to the main menu.");
            return;
        }

        // Normalize the path, truncating any file name before updating LAST_DIR_USED.
        let purepath = dir_normalize_path(readpath.as_str());
        *LAST_DIR_USED.lock().unwrap() = purepath.clone(); // Update LAST_DIR_USED

        //endregion

        // region Read the chosen file.

        let usebank: Bank;
        match file_read_to_string(&readpath) {
            Ok(contents) => {
                usebank = serde_json::from_str(&contents).unwrap();
                *CURRENT_BANK.lock().unwrap() = usebank.clone();
            }
            // TODO: Fix error handling.  This is terrible.  See thread in forum at
            // https://users.rust-lang.org/t/help-understanding-never-used-warning/125562/2
            Err(err) => {
                eprintln!("\n Error reading the file: {} \n", err);
                fltk_custom_message("Error reading the file.","Return to the main menu.");
            }
        }
        // endregion
    }

    /// Refreshes the contents of the title box of a bank's display.
    ///
    pub fn bnk_refresh_title() {
        let usebank: Bank;
        let mut wdgts: Wdgts;
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
            wdgts = WIDGETS.lock().unwrap().clone();
        } // Access global structs.

        let mut buf = TextBuffer::default();
        buf.set_text(usebank.bank_title.as_str()); // Uses the title from the current bank.
        wdgts.title_editbox.set_buffer(buf);

        //  let title_text =   // There is likely to be a use for  title_text   in the future.
        wdgts.title_editbox.buffer().unwrap().text();
    }

    /// Prepares a Bank struct for saving.
    ///
    pub fn bnk_save() {

        let lastdir = glob_check_lastdirused();  // Handles the case where LAST_DIR_USED is empty.

        let usebank: Bank;   // Pull the data from CURRENT_BANK.
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
        }

        let usename = usebank.bank_title.clone();  // Pull the bank title to be used as the file name.
        let usepath = file_browse_tosave(&lastdir, usename.as_str(), "*.bnk");  // Browse to choose directory and set file name.

        {  // Since the new path has been chosen, update LAST_DIR_USED.
            let purepath = dir_normalize_path(usepath.as_str());  // Normalize the path and truncate any file name.
            *LAST_DIR_USED.lock().unwrap() = purepath.clone();
        }

        bnk_save_as_json(&usepath);
        // endregion
    }

    /// Saves a Bank struct to a file in json format.
    ///
    pub fn bnk_save_as_json(usepath: &String) {
        let usebank;
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
        }

        let bnk_as_json = serde_json::to_string(&usebank).unwrap(); // Convert bank to json string.

        let mut file = File::create(usepath).expect("Could not create file!");

        file.write_all(bnk_as_json.as_bytes())
            .expect("Cannot write to the file!");
    }

    /// Recalculates the variables in the questions of a Bank.
    ///
    pub fn bnk_recalc() {
        let usebank;
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
        }

        println!("\n Recalculate variables in a bank.  Not yet implemented. \n");
        println!("\n The current bank is: \n {:?} \n", usebank);

        //  Read the passed bank
        //  for q in usebank.question_vec.iter() {
        //      step thru and recalc each variable
        //      in each question.
        // }
    }
    
} // End    bank    module

/// Functions that deal with the Question struct.
///
pub mod questions {
    use crate::banks::{bnk_refresh_widgets, bnk_save, Bank};
    use crate::variable::*;
    use crate::{APP_FLTK, CURRENT_BANK, LAST_DIR_USED, VARIABLE_DIR};
    use fltk::app::set_font_size;
    use fltk::enums::{Color, Shortcut};
    use fltk::prelude::{DisplayExt, GroupExt, MenuExt, WidgetBase, WidgetExt, WindowExt};
    use fltk::text::{TextBuffer, TextEditor};
    use fltk::{app, menu, text, window};
    use lib_file::file_fltk::*;
    use lib_file::file_mngmnt::file_get_dir_list;
    use lib_myfltk::fltkutils::*;
    use lib_myfltk::input_fltk::{input_string, input_strvec};
    use lib_utils::utilities::*;
    use serde::{Deserialize, Serialize};
    use crate::global::glob_check_lastdirused;
    //region Struct Section

    /// The second layer of the three structs QBC is built around.
    /// Note that the field `var_vec: Vec<Variable>` is a vector
    /// of type `Variable`.  `Variable` is the third and innermost layer in the
    /// three-struct nest.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Question {
        pub qtext: String,
        pub var_dirpath: String,
        pub var_vec: Vec<Variable>,
        pub answer: String,
        pub objectives: Vec<String>,
        pub prereqs: Vec<String>,
    }

    impl Question {
        /// Initialize a new Question.
        fn new() -> Question {
            Self {
                qtext: "Please enter the text of your question.  Use real values. You will replace those values with variables later.  Be sure to delete these instructions before entering your question text.".to_string(),
                var_dirpath: VARIABLE_DIR.to_string(),
                var_vec: Vec::new(),
                answer: "Answer".to_string(),
                objectives: Vec::new(),
                prereqs: Vec::new(),
            }
        }
    } // End   Question   impl
      //endregion

    /// Create a new question.
    ///
    pub fn qst_create() {
        // todo: The answer will need to parse inserted variables.

        let mut newquest = Question::new();

        // region Question data entry

        let nowtext = qst_editor(newquest.qtext.as_str(), "Question Editor");
        newquest.qtext = nowtext.clone();

        // Pull the flagged variables from the text and push them to the variable vector.
        qst_fill_varvec_parsetext(&mut newquest);

        // Answer will eventually need to be calculated.

        let app;
        {
            app = *APP_FLTK.lock().unwrap();
        }

        newquest.answer = input_string(&app, "Please input the question's answer:  ", 790, 300);
        newquest.objectives =
            input_strvec(&app, "Please enter the question objectives:  ", 790, 300);
        newquest.prereqs =
            input_strvec(&app, "Please enter the question prerequisites:  ", 790, 300);
        // endregion

        // region Save and store the data
        let mut usebank: Bank;
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
        } // Access the global Bank variable
        usebank.question_vec.push(newquest); // Store the new question in the bank
        {
            // Pass the modified bank into the global variable.
            *CURRENT_BANK.lock().unwrap() = usebank.clone();
        }
        bnk_save();
        // endregion
    }

    /// Edit a question.
    ///
    pub fn qst_edit(qst_idx: usize) {
        let app;
        let mut usebank;
        {
            app = *APP_FLTK.lock().unwrap();
            usebank = CURRENT_BANK.lock().unwrap().clone();
        } // Access global variables.

        let mut editqst = usebank.question_vec.get(qst_idx).unwrap().clone();

        let nowtext = qst_editor(editqst.qtext.as_str(), "Question Editor");
        editqst.qtext = nowtext.clone();

        // Pull the flagged variables from the text and push them to the variable vector.
        qst_fill_varvec_parsetext(&mut editqst); // Need to clear the vector first.

        // Answer will eventually need to be calculated.

        editqst.answer = input_string(&app, "Please input the question's answer:  ", 790, 300);
        editqst.objectives =
            input_strvec(&app, "Please enter the question objectives:  ", 790, 300);
        editqst.prereqs =
            input_strvec(&app, "Please enter the question prerequisites:  ", 790, 300);

        // Push the question to the vector in the bank and save the bank.
        //let mut usebank = CURRENT_BANK.lock().unwrap();

        // todo: This won't work.  push()  appends to the end of the vector. Fix it.
        usebank.question_vec.push(editqst.clone());
        bnk_save();
        bnk_refresh_widgets();
    }

    /// // Is this necessary now?
    ///
    pub fn qst_chooseqst() -> Question {
        // TODO: Instead of trying to put the whole text of the question
        //          body in the radio button, number each question in the
        //          bank display and choose by the question number.

        // Note:  This function may not be necessary.

        let mut usevec: Vec<String> = Vec::new();

        let usebank;
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
        }

        for item in usebank.question_vec.iter() {
            usevec.push(item.qtext.clone());
        }

        let usequest = fltk_radio_lightbtn_menu(&usevec, "");
        let mut editquest = Question::new();

        for item in usebank.question_vec.iter() {
            if item.qtext == usequest {
                editquest = item.clone();
            }
        }

        editquest
    }

    /// Parses the text of a question looking for flags that mark
    /// variables.  Separates out the flagged variables, reading the
    /// data into Variable structs and saving them in the
    /// `quest.var_vec` vector field of the current question.
    pub fn qst_fill_varvec_parsetext(quest: &mut Question) {
        // Note:  All variable files must be in the same directory.

        // region Create a vector of the variable names that have been flagged in the text.
        let mut usevec = util_flaggedtxt_2vec(&quest.qtext, '§');
        usevec.sort();
        usevec.dedup(); // Remove repeats of the flagged variable names.
        // endregion

        // region Read the variable files from disk and insert them into the variable vector.

        let lastdir = glob_check_lastdirused();
        let usepath = file_pathonly(&lastdir, "Choose the folder that contains your variable files.");

        quest.var_vec.clear();
        for item in usevec.iter() {
            match vrbl_read_with_pathfname(&usepath, item) {
                Some(newvar) => {
                    quest.var_vec.push(newvar);
                }
                None => {
                    eprintln!("\n Error reading the Variable file {}. \n", item);
                    fltk_custom_message("Error reading the Variable file.","Return to the question editor.");
                }
            }
        }

            // todo:  Add a check to make sure all needed variable files are in the same directory.
            // todo:  Add a check to make sure those variable files exist and are readable.
            // todo:  For later.  Allow for the variable files to be in different directories.

        // endregion
    }

    /// Calls up an FLTK TextEditor for entering/editing question text
    /// and variables.
    pub fn qst_editor(startertxt: &str, winlabel: &str) -> String {
        let mut buf = TextBuffer::default();
        let mut edtrwin = window::Window::default().with_size(800, 300);
        set_font_size(20);
        edtrwin.set_color(Color::Blue);
        edtrwin.set_label(winlabel);
        edtrwin.make_resizable(true);

        buf.set_text(startertxt);
        let mut edtr = TextEditor::default().with_size(770, 222).center_of_parent();

        qst_editor_menubar(&edtr, &mut edtrwin, &mut buf);

        edtr.set_buffer(buf.clone()); // Clone is used here to avoid an ownership error.
        edtr.wrap_mode(text::WrapMode::AtBounds, 0);
        edtr.set_color(Color::White);
        edtr.set_text_size(22);
        edtr.set_text_color(Color::Black);

        edtrwin.end();
        edtrwin.show();

        while edtrwin.shown() {
            app::wait();
        }

        println!(
            "\n W5:  End of qst_editor().  The quesion text is:  {} \n",
            buf.text()
        );

        buf.text()
    }

    /// Menu bar for the `qst_editor`.
    ///
    pub fn qst_editor_menubar(edtr: &TextEditor, edtrwin: &mut window::Window, buf: &mut TextBuffer) -> menu::MenuBar {
        let mut menubar = menu::MenuBar::new(0, 0, edtrwin.width(), 40, "");

        // region  "Finished" menu item
        let mut edtrwin_clone = edtrwin.clone();
        let quit_idx = menubar.add(
            "Finished\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                edtrwin_clone.hide();
            },
        );
        menubar.at(quit_idx).unwrap().set_label_color(Color::Red);
        // endregion

        // region "Insert Variable" menu item
        let edtr_clone = edtr.clone();
        let mut buf_clone = buf.clone();
        menubar.add(
            "Insert_Variable\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                let newtext = qst_make_var_replace_text();
                fltk_replace_highlighted_text(&edtr_clone, &mut buf_clone, &newtext);
            },
        );
        // endregion

        menubar
    }

    /// This function is called when the user highlights text that is
    /// to be replaced by a variable. This function uses the variable name
    /// to create text -- between flags -- that then replaces the highlighted text.
    /// Returns the replacement text.
    pub fn qst_make_var_replace_text() -> String {
        // todo: Allow for user to input a more readable variable name
        //          than the name of the variable file name on disk.
        // todo: Change the display of the variable name to be more readable without using
        //          flags.

        let lastdir = glob_check_lastdirused();

        let path = file_pathonly(&lastdir, "Choose the folder that contains your variable files.");
        {
            LAST_DIR_USED.lock().unwrap().clone_from(&path); // Refresh LAST_DIR_USED
        }

        let flist = file_get_dir_list(&path);
        let varname = fltk_radio_lightbtn_menu(&flist, "");
        let rpltxt = format!("§{}§", varname);

        rpltxt
    }




    /*
       -- Answers will be calculated from the current variable values.

       -- What are you going to do about operators and how they interact, especially
           when the operator is given in the question in verbal format?
           -- The answer equation will have to be entered by the user.

       -- And then there be equations!!!!

    */ // Issues & questions

} // End   questions   module

/// Functions that deal with the Variable struct.
///
pub mod variable {

    use crate::global::{glob_check_lastdirused, TypeWrapper, TypeWrapper::*};
    use crate::{lists::*, math_functions::*, CURRENT_BANK, LAST_DIR_USED, VARIABLE_DIR};
    use fltk::app;
    use fltk::button::{Button, CheckButton, RadioLightButton};
    use fltk::enums::{Color, FrameType};
    use fltk::input::{FloatInput, IntInput};
    use fltk::prelude::{ButtonExt, GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt};
    use fltk::{frame::Frame, group::Group, window::Window};
    use lib_file::{file_fltk::*, file_mngmnt::*};
    use lib_utils::vec::*;
    use serde::{Deserialize, Serialize};
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::{fs::File, io::Write};
    use fltk::dialog::{choice2_default, message_title};
    use lib_file::dir_mngmnt::dir_normalize_path;
    use lib_myfltk::fltkutils::fltk_custom_message;
    use crate::banks::{bnk_loaded, bnk_save, Bank};
    use crate::misc::dir_is_empty;

    //region Struct Section

    /// The third and innermost layer of the three struct nest
    /// that QBC is built around.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Variable {
        pub var_fname: String,
        pub display_name: String,
        pub params: VarPrmtrs,
        pub list_fname: String,
        pub content: TypeWrapper,
        pub var_type: String,
    }

    impl Default for Variable {
        fn default() -> Self {
            Self {
                var_fname: "New_Variable".to_string(),
                display_name: "New_Variable".to_string(),
                params: VarPrmtrs::new(),
                list_fname: "".to_string(),
                content: Integer(0),
                var_type: "Strings".to_string(), // "Strings", "chars", "ints", "floats"
            }
        }
    }

    impl Variable {
        /// Initialize a new Variable.
        ///
        pub fn new() -> Variable {
            Self::default()
        }

    } // End Variable impl

    /// Struct that holds the parameters that determine the behavior
    /// of a Variable.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct VarPrmtrs {
        pub is_string: bool,
        pub is_char: bool,
        pub is_from_list: bool,
        pub is_int: bool,
        pub is_float: bool,
        pub num_min_int: i64,
        pub num_max_int: i64,
        pub num_min_float: f64,
        pub num_max_float: f64,
        pub num_dcml_places: usize,
        pub num_comma_frmttd: bool,
    }

    impl Default for VarPrmtrs {
        fn default() -> Self {
            Self::new()
        }
    }

    impl VarPrmtrs {
        /// Creates a new variable.  Sets the
        /// default variable type to `int` and all ranges to 0.
        pub fn new() -> VarPrmtrs {
            Self {
                is_string: false,
                is_char: false,
                is_from_list: false,
                is_int: true,
                is_float: false,
                num_min_int: 0,
                num_max_int: 0,
                num_min_float: 0.0,
                num_max_float: 0.0,
                num_dcml_places: 0,
                num_comma_frmttd: false, // Leave implementing this until you need to output it.

                                         // Default values all assume that the variable is an i64.
            }
        }
    } // ~~~~~ End VarPrmtrs impl ~~~~~

      //endregion

    /// Create a new variable.
    ///
    pub fn vrbl_create() {
        //todo: Instead of passing a reference to the variable struct,
        //          have the parameters box return the variable struct.

        let mut var1 = Variable::new();

        vrbl_parameters_input_box(&mut var1);

        println!(
            "\n W3 -- Back in vrbl_create():  The Variable struct now contains:  {:?} \n",
            var1
        );

        vrbl_save(&mut var1);
    }

    /// Input and save a new variable's parameters into the Variable struct.
    ///
    pub fn vrbl_parameters_input_box(var1: &mut Variable) {
        // todo: Grey out the input fields when the variable type is not "int" or "float".
        //          Use the `deactivate()` method.  First attempt didn't work.

        let mut win = Window::new(900, 100, 600, 400, "Variable Parameters");
        win.set_color(Color::Cyan);
        win.make_resizable(true);

        // region Create the radio buttons for the variable type.
        let radio_group = Group::new(0, 0, 600, 50, None);

        // Create horizontal radio light buttons across the top -- initial spacing.
        let bttn_w = 120;
        let bttn_h = 30;
        let spacing = 20;
        let types_xxx = 40;
        let types_yyy = 20;

        let mut strings_btn =
            RadioLightButton::new(types_xxx, types_yyy, bttn_w, bttn_h, "Strings");
        let chars_btn = RadioLightButton::new(
            types_xxx + bttn_w + spacing,
            types_yyy,
            bttn_w,
            bttn_h,
            "Characters",
        );
        let ints_btn = RadioLightButton::new(
            types_xxx + 2 * (bttn_w + spacing),
            types_yyy,
            bttn_w,
            bttn_h,
            "Integers",
        );
        let decimals_btn = RadioLightButton::new(
            types_xxx + 3 * (bttn_w + spacing),
            types_yyy,
            bttn_w,
            bttn_h,
            "Decimals",
        );

        // Set Integers as default selection
        strings_btn.set_value(true);

        radio_group.end();
        // endregion

        // region Create "comma" & "list" check boxes in row below the radio buttons.

        // Calculate the position & size of the check boxes.
        let ckbx_y = types_yyy + bttn_h + 20; // Position below radio buttons
        let ckbx_w = 150;
        let ckbx_h = 25;
        let ckbx_spacing = 55;

        let total_radio_width = bttn_w * 4 + spacing * 2; // Width of all radio buttons + spacing
        let start_x = types_xxx + (total_radio_width - (ckbx_w * 2 + ckbx_spacing)) / 2;

        // Create the check boxes.
        let usecommas = CheckButton::new(start_x, ckbx_y, ckbx_w, ckbx_h, "Comma Formatted");
        let fromlist = CheckButton::new(
            start_x + ckbx_w + ckbx_spacing,
            ckbx_y,
            ckbx_w,
            ckbx_h,
            "Value to come from a List",
        );

        // endregion

        // region Set up frames -- for Integer & Decimal parameter entry.

        // region Set up frame parameters
        let frame_y = ckbx_y + ckbx_h + 20; // Position below checkboxes
        let frame_w = 250;
        let input_w = 100;
        let input_h = 25;
        let label_h = 20;
        let field_spacing = 10;
        let frame_spacing = 20;
        let frame_h = 30 + (3 * (label_h + input_h + field_spacing)) + 15;
        // endregion

        // region Create Integers frame & input fields
        let mut int_frame = Group::new(types_xxx, frame_y, frame_w, frame_h, None);
        let mut int_label = Frame::new(types_xxx, frame_y, frame_w, 30, "Integer Parameters");
        int_label.set_label_size(14);

        // Calculate centered position for input fields in integer frame
        let int_input_x = types_xxx + (frame_w - input_w) / 2;
        let int_first_y = frame_y + 35; // Start below the frame label

        // Integer Minimum Value
        let _intmin_label = Frame::new(int_input_x, int_first_y, input_w, label_h, "Minimum Value");
        let mut intmin = IntInput::new(int_input_x, int_first_y + label_h, input_w, input_h, "");

        // Integer Maximum Value
        let _intmax_label = Frame::new(
            int_input_x,
            int_first_y + label_h + input_h + field_spacing,
            input_w,
            label_h,
            "Maximum Value",
        );
        let mut intmax = IntInput::new(
            int_input_x,
            int_first_y + label_h + input_h + field_spacing + label_h,
            input_w,
            input_h,
            "",
        );

        int_frame.set_frame(FrameType::DownBox); // Add frame border
        int_frame.end();
        // endregion

        // region Create Decimals frame & input fields
        let mut decimal_frame = Group::new(
            types_xxx + frame_w + frame_spacing,
            frame_y,
            frame_w,
            frame_h,
            None,
        );
        let mut decimal_label = Frame::new(
            types_xxx + frame_w + frame_spacing,
            frame_y,
            frame_w,
            30,
            "Decimal Parameters",
        );
        decimal_label.set_label_size(14);

        // Calculate centered position for input fields in decimal frame
        let dec_input_x = types_xxx + frame_w + frame_spacing + (frame_w - input_w) / 2;
        let dec_first_y = frame_y + 35; // Start below the frame label

        // Decimal Minimum Value
        let _decmin_label = Frame::new(dec_input_x, dec_first_y, input_w, label_h, "Minimum Value");
        let mut decmin = FloatInput::new(dec_input_x, dec_first_y + label_h, input_w, input_h, "");

        // Decimal Maximum Value
        let _decmax_label = Frame::new(
            dec_input_x,
            dec_first_y + label_h + input_h + field_spacing,
            input_w,
            label_h,
            "Maximum Value",
        );
        let mut decmax = FloatInput::new(
            dec_input_x,
            dec_first_y + label_h + input_h + field_spacing + label_h,
            input_w,
            input_h,
            "",
        );

        // Decimal Places
        let _decplaces_label = Frame::new(
            dec_input_x,
            dec_first_y + 2 * (label_h + input_h + field_spacing),
            input_w,
            label_h,
            "Decimal Places",
        );
        let mut decplaces = IntInput::new(
            dec_input_x,
            dec_first_y + 2 * (label_h + input_h + field_spacing) + label_h,
            input_w,
            input_h,
            "",
        );

        decimal_frame.set_frame(FrameType::DownBox); // Add frame border
        decimal_frame.end();
        // endregion

        // endregion

        // region Create the Submit button
        let submit_btn_w = 100;
        let submit_btn_h = 40;

        // Calculate center position based on the frames
        let total_frames_width = frame_w * 2 + frame_spacing;
        let submit_btn_x = types_xxx + (total_frames_width - submit_btn_w) / 2;
        let submit_btn_y = frame_y + frame_h + 20; // 20 pixels gap after frames

        let mut submit_btn = Button::new(
            submit_btn_x,
            submit_btn_y,
            submit_btn_w,
            submit_btn_h,
            "Submit",
        );
        // endregion

        win.end();
        win.show();

        // region Clone variables for the callback
        let strings_btn = strings_btn.clone();
        let chars_btn = chars_btn.clone();
        let ints_btn = ints_btn.clone();
        let decimals_btn = decimals_btn.clone();
        let mut win_clone = win.clone();

        let datavar = Rc::new(RefCell::new(Variable::new()));
        let datavar_outside = datavar.clone(); // Create a second Rc pointing to the same RefCell

        // endregion

        // region Do the callback for the Submit button
        submit_btn.set_callback(move |_| {
            // region Deal with the radio buttons.
            let vartype = if strings_btn.value() {
                decmin.deactivate();
                decmax.deactivate();
                decplaces.deactivate();
                intmin.deactivate();
                intmax.deactivate();
                "Strings"
            } else if chars_btn.value() {
                decmin.deactivate();
                decmax.deactivate();
                decplaces.deactivate();
                intmin.deactivate();
                intmax.deactivate();
                "Characters"
            } else if ints_btn.value() {
                decmin.deactivate();
                decmax.deactivate();
                decplaces.deactivate();
                "Integers"
            } else if decimals_btn.value() {
                intmin.deactivate();
                intmax.deactivate();
                "Decimals"
            } else {
                "None"
            };

            datavar.borrow_mut().var_type = vartype.to_string();
            // endregion

            //region Deal with the "comma" & "list" check boxes.
            if usecommas.is_checked() {
                datavar.borrow_mut().params.num_comma_frmttd = true;
                print!("\n Comma Formatted == true \n");
            } else {
                datavar.borrow_mut().params.num_comma_frmttd = false;
                print!("\n Comma Formatted == false \n");
            }

            if fromlist.is_checked() {
                datavar.borrow_mut().params.is_from_list = true;
                print!("\n List == true \n");
            } else {
                datavar.borrow_mut().params.is_from_list = false;
                print!("\nList == false \n");
            }
            // endregion

            // region Deal with the Integer input fields.
            if vartype == "Integers" {
                datavar.borrow_mut().params.is_int = true;
                datavar.borrow_mut().params.is_float = false;
                datavar.borrow_mut().params.num_min_int = intmin.value().parse::<i64>().unwrap();
                datavar.borrow_mut().params.num_max_int = intmax.value().parse::<i64>().unwrap();
            }
            // endregion

            // region Deal with the Decimal input fields.
            if vartype == "Decimals" {
                datavar.borrow_mut().params.is_int = false;
                datavar.borrow_mut().params.is_float = true;
                datavar.borrow_mut().params.num_min_float = decmin.value().parse::<f64>().unwrap();
                datavar.borrow_mut().params.num_max_float = decmax.value().parse::<f64>().unwrap();
                datavar.borrow_mut().params.num_dcml_places =
                    decplaces.value().parse::<usize>().unwrap();
            }
            // endregion

            println!("\n In the callback, datavar == {:?} \n", datavar);

            // Close the window
            win_clone.hide();
        });
        // endregion

        // Keep window active until hidden
        while win.shown() {
            app::wait();
        }

        *var1 = datavar_outside.borrow().clone();
    }

    /// Prepare a Variable for saving.
    ///
    pub fn vrbl_save(var1: &mut Variable) {

        let lastdir = glob_check_lastdirused();

        let usepath = file_browse_tosave(&lastdir, "",
          "Variable Files   \t*.vrbl\nText Files   \t*.txt\nList Files    \t*.lst\nAll Files    \t*.*");

        {  // Set LAST_DIR_USED to the new path.
            let purepath: String = dir_normalize_path(&usepath);
            *LAST_DIR_USED.lock().unwrap() = purepath.clone();
        }

        var1.var_fname = file_path_to_fname(&usepath);
        vrbl_save_as_json(var1, &usepath);

        println!("\n The variable has been saved \n");
    }

    /// Save a Variable in json format.
    ///
    pub fn vrbl_save_as_json(var: &Variable, usepath: &str) {
        let var_as_json = serde_json::to_string(var).unwrap();

        let mut file = File::create(usepath).expect("Could not create file!");

        file.write_all(var_as_json.as_bytes())
            .expect("Cannot write to the file!");
    }

    /// Read a variable from a file.
    ///
    pub fn vrbl_read() -> Option<Variable> {

        // region Deal with the directory path

        let lastdir = glob_check_lastdirused();
        let readpath: String;  // This will be the path to the file that is to be read.

        let usepath = file_pathonly(&lastdir, "Choose the folder where you save your Variable files." );

        {  // Set LAST_DIR_USED to the new path.
            let purepath: String = dir_normalize_path(&usepath);
            *LAST_DIR_USED.lock().unwrap() = purepath.clone();
        }

        if !dir_is_empty(&usepath) {
            readpath = file_fullpath(&usepath, "Choose the Variable file you want to read.");
        } else {
            fltk_custom_message("The directory you chose is empty.","Return to program.");
            return None;
        }

        // endregion

        //region Read the file & return the Variable struct

        match file_read_to_string(&readpath) {
            Ok(contents) => {
                serde_json::from_str(&contents).unwrap()
            }
            Err(err) => {
                eprintln!("\n Error reading the file: {} \n", err);
                fltk_custom_message("Could not read the file.","Return to the question editor.");
                None
            }
        }

        // endregion
    }

    /// Read a variable from a file.
    ///
    pub fn vrbl_read_with_pathfname(usepath: &str, fname: &str) -> Option<Variable> {
    // Note:  This function requires that the usepath & fname parameters
    //          have already been validated & normalized before passing them
    //          to the function.

        let readpath = format!("{}/{}", usepath, fname);

        match file_read_to_string(&readpath) {
            Ok(contents) => {
                serde_json::from_str(&contents).unwrap()
            }
            Err(err) => {
                eprintln!("\n Error reading the file: {} \n", err);
                fltk_custom_message("Could not read the file.","Return to the question editor.");
                None
            }
        }
    }

    /// Sets and calculates the values of non-boolean fields in the Variable struct.
    ///
    pub fn vrbl_setvalues(var1: &mut Variable) {
        if var1.params.is_from_list {  // If the variable content comes from a list.

            // todo: Delete the debug `println!` statements.

            match var1.var_type.as_str() { // Set the variable content field.
                "Strings" => {
                    let read_optn = list_read("Strings");
                    match read_optn {
                        Some((fname, newlist)) => {
                            var1.list_fname = fname; // Sets the value of the variable's listname field
                            let usevec = newlist.words.clone(); // Clones the list content vector so you can mess with it.
                            let item = vec_random_choice(&usevec);
                            match item {
                                Some(x) => {
                                    println!("\n The chosen item is:  {:?}", x);
                                    var1.content = Alphanum(x.0.to_string());
                                }
                                None => {
                                    fltk_custom_message("No item was chosen.","Return.");
                                    eprintln!("\n The function `vec_random_choice()` returned `None`. \n");
                                    return;
                                },
                            }

                        }
                        None => {
                            eprintln!("No list file selected.");
                            fltk_custom_message("No list file selected.","Return to the question editor.");
                            return;
                        }

                    }
                }

                "chars" => {
                    let read_optn = list_read("chars");
                    match read_optn {
                        Some((fname, newlist)) => {
                            var1.list_fname = fname; // Sets the value of the variable's listname field
                            let usevec = newlist.runes.clone(); // Clones the list content vector so you can mess with it.
                            let item = vec_random_choice(&usevec);
                            match item {
                                Some(x) => {
                                    println!("\n The chosen item is:  {:?}", x);
                                    var1.content = Letter(*x.0);
                                }
                                None => {
                                    fltk_custom_message("No item was chosen.","Return.");
                                    eprintln!("\n The function `vec_random_choice()` returned `None`. \n");
                                    return;
                                },
                            }

                        }
                        None => {
                            eprintln!("No list file selected.");
                            fltk_custom_message("No list file selected.","Return to the question editor.");
                            return;
                        }

                    }
                }

                "ints" => {
                    let read_optn = list_read("ints");
                    match read_optn {
                        Some((fname, newlist)) => {
                            var1.list_fname = fname; // Sets the value of the variable's listname field
                            let usevec = newlist.intsigned.clone(); // Clones the list content vector so you can mess with it.
                            let item = vec_random_choice(&usevec);
                            match item {
                                Some(x) => {
                                    println!("\n The chosen item is:  {:?}", x);
                                    var1.content = Integer(*x.0);
                                }
                                None => {
                                    fltk_custom_message("No item was chosen.", "Return.");
                                    eprintln!("\n The function `vec_random_choice()` returned `None`. \n");
                                    return;
                                },
                            }
                        }
                        None => {
                            eprintln!("No list file selected.");
                            fltk_custom_message("No list file selected.", "Return to the question editor.");
                            return;
                        }
                    }
                }

                "floats" => {

                    let read_optn = list_read("floats");
                    match read_optn {
                        Some((fname, newlist)) => {
                            var1.list_fname = fname; // Sets the value of the variable's listname field
                            let usevec = newlist.decimals.clone(); // Clones the list content vector so you can mess with it.
                            let item = vec_random_choice(&usevec);
                            match item {
                                Some(x) => {
                                    println!("\n The chosen item is:  {:?}", x);
                                    var1.content = Floating(*x.0);
                                }
                                None => {
                                    fltk_custom_message("No item was chosen.", "Return.");
                                    eprintln!("\n The function `vec_random_choice()` returned `None`. \n");
                                    return;
                                },
                            }
                        }
                        None => {
                            eprintln!("No list file selected.");
                            fltk_custom_message("No list file selected.", "Return to the question editor.");
                            return;
                        }
                    }
                }

                _ => {}
            }



        } else if var1.params.is_int {
            // Numeric values will always be randomly generated.
            let numint: i64 =
                math_gen_random_num(var1.params.num_min_int, var1.params.num_max_int);
            var1.content = Integer(numint);
        } else {
            // The content is a float.
            let mut numfloat: f64 =
                math_gen_random_num(var1.params.num_min_float, var1.params.num_max_float);
            numfloat = math_round_to_place_f64(&numfloat, var1.params.num_dcml_places);
            var1.content = Floating(numfloat);
        }
    }

    /// Recalculates the values in the non-boolean fields of a Variable struct.
    ///
    pub fn vrbl_recalc() {
        match vrbl_read() {
            Some(mut vrbl) => {

                println!("\n The variable before recalc is: \n {:?}", vrbl);

                vrbl_setvalues(&mut vrbl);

                println!("\n The variable after recalc is: \n {:?} \n", vrbl);

                vrbl_save(&mut vrbl);
            }
            None => {}
        }

        //let mut usevar = vrbl_read();

        //vrbl_setvalues(&mut usevar);
        //vrbl_save(&mut usevar);

    }

} // End   variable   module

/// Functions for creating and manipulating lists.
///
pub mod lists {

    use crate::{APP_FLTK, LAST_DIR_USED, LIST_DIR, VARIABLE_DIR};
    use lib_file::file_fltk::{file_browse_tosave, file_fullpath, file_fullpath_fltr, file_pathonly};
    use lib_file::file_mngmnt::file_read_to_string;
    use lib_myfltk::input_fltk::*;
    use serde::{Deserialize, Serialize};
    use std::{fs::File, io::Write};
    use lib_file::dir_mngmnt::dir_normalize_path;
    use lib_myfltk::fltkutils::fltk_custom_message;
    use crate::global::glob_check_lastdirused;
    use crate::misc::dir_is_empty;
    // region Struct section

    /// Contains a vector field for each of the four data types
    /// allowed in lists associated with QBC.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct List {
        pub words: Vec<String>,
        pub runes: Vec<char>,
        pub intsigned: Vec<i64>,
        pub decimals: Vec<f64>,
        pub typechoice: String, // "Strings", "chars", "ints", "floats"
    }

    impl Default for List {
        fn default() -> Self {
            Self::new()
        }
    }

    impl List {
        /// Initializes a new list struct.
        pub fn new() -> List {
            Self {
                words: Vec::new(),
                runes: Vec::new(),
                intsigned: Vec::new(),
                decimals: Vec::new(),
                typechoice: "Strings".to_string(),
            }
        }
    } // ----------  End List impl ----------

    // endregion

    /// Create a new list.
    ///
    pub fn list_create(typech: &str) {
        let mut newlist = List::new();
        newlist.typechoice = typech.to_string();

        let app;
        {
            app = APP_FLTK.lock().unwrap();
        }

        match typech {
            "String" | "Strings" => {
                // String
                let uselist = input_strvec(&app, "Please enter a string.", 790, 300);
                newlist.words = uselist.clone();
                list_save(&newlist);
            }

            "char" | "chars" => {
                // char
                let uselist = input_charvec(&app, "Please enter a character.");
                newlist.runes = uselist.clone();
                list_save(&newlist);
            }

            "int" | "ints" => {
                // i64
                let uselist = input_i64vec(&app, "Please enter an integer.");
                newlist.intsigned = uselist.clone();
                list_save(&newlist);
            }

            "float" | "floats" => {
                // f64

                let uselist = input_f64vec(&app, "Please enter a floating point number.");
                newlist.decimals = uselist.clone();
                list_save(&newlist);
            }

            _ => {
                panic!("\n\n No match found!  Fix it!!\n\n");
            }
        }

        // Note that the function saves the list, but does not return it.
    }

    /// Read a list (in json format) from a file.  Returns a tuple (filename, List)
    /// containing the file name that was read along with the reconstituted list.
    pub fn list_read(typech: &str) -> Option<(String, List)> {  // Do you really need to return the filename?

        // region Deal with the directory path
        let lastdir = glob_check_lastdirused();
        let usepath = file_pathonly(&lastdir, "Choose the folder where you save your List files." );

        {  // Set LAST_DIR_USED to the new path.
            let purepath: String = dir_normalize_path(&usepath);
            *LAST_DIR_USED.lock().unwrap() = purepath.clone();
        }

        // endregion

        //region Read the file & return the List struct

        let readlist = loop {

            if dir_is_empty(&usepath) {
                fltk_custom_message("The directory you chose is empty.","Return to program.");
                return None;
            }

            {  // Set LAST_DIR_USED to the new path.
                let purepath: String = dir_normalize_path(&usepath);
                *LAST_DIR_USED.lock().unwrap() = purepath.clone();
            }

            let uselistname = file_fullpath_fltr(&usepath, "*.lst");

            match file_read_to_string(&uselistname) {
                Ok(contents) => {
                    let newlist = serde_json::from_str(&contents).unwrap();
                    let typechk = list_check_typematch(&newlist, typech);
                    if !typechk {
                        continue;
                    } else {
                        break (uselistname, newlist);
                    }
                }
                Err(err) => {
                    eprintln!("\n Error reading the file: {} \n", err);
                    fltk_custom_message("Could not read the file.","Return.");
                    return None;
                }
            }
        };
        // endregion

        Some(readlist)
    }

    /// Edit an existing list.  Not yet implementd
    ///
    pub fn list_edit() {
        println!("\n Someday I'll write this function. \n");
    }

    /// Prepare a list for saving to a file.
    ///
    pub fn list_save(list: &List) -> String {

        println!("\n WL1 First waypoint in list_save(). \n");

        // region Set up directories.

        let mut usedir = String::new();
        {
            if LAST_DIR_USED.lock().unwrap().clone() == "" {
                *LAST_DIR_USED.lock().unwrap() = LIST_DIR.to_string().clone();
            } // If there is no recently used directory, use default.

            println!("\n WL2 Second waypoint in list_save(). \n");

            usedir = LAST_DIR_USED.lock().unwrap().clone();

            println!("\n WL3 Third waypoint in list_save(). \n");
        }
        // endregion

        //region Call the file browser to get the proper path.

        println!("\n WL4 Fourth waypoint in list_save(). \n");

        let path = file_browse_tosave(&usedir, "",
            "List Files    \t*.lst\nVariable Files   \t*.vrbl\nText Files   \t*.txt\nAll Files",
        );

        println!("\n WL5 Fifth waypoint in list_save(). \n");

        {
            *LAST_DIR_USED.lock().unwrap() = path.clone(); // Store the current path in global.
        }
        // endregion

        println!("\n WL6 Sixth waypoint in list_save(). \n");

        list_save_as_json(list, path.as_str());

        path
    }

    /// Save a list in json format.
    ///
    pub fn list_save_as_json(list: &List, fname: &str) {
        let list_as_json = serde_json::to_string(list).unwrap();

        let mut file = File::create(fname).expect("Could not create file!");

        file.write_all(list_as_json.as_bytes())
            .expect("Cannot write to the file!");
    }

    /// Check that a list contains the correct type of data.
    ///
    pub fn list_check_typematch(uselist: &List, typech: &str) -> bool {
        if uselist.typechoice.as_str() != typech {
            println!("\n The data type of that list does not match your typechoice. \n");
            println!("Please choose a different list file. \n");
            false
        } else {
            true
        }
    }

} // End  lists module

/// Functions for use in creating menus.
///
pub mod menus {
    use crate::misc::check_for_bank_loaded;
    use crate::{banks::*, lists::*, questions::*, variable::*};
    use fltk::enums::{Color, Shortcut};
    use fltk::prelude::{MenuExt, WidgetBase, WidgetExt};
    use fltk::{app::quit, menu, window::Window};

    /// Create a menubar for the primary window.
    pub fn qbnk_menubar(primwin: &mut Window) -> menu::MenuBar {
        // todo: `primwin` is in the global Widgets variable.  Access it
        //          there rather than passing it to this function.

        let mut menubar = menu::MenuBar::new(0, 0, primwin.width(), 40, "");

        //region File section

        menubar.add(
            "File/Print/Question Bank\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| println!("Printing a new Question Bank."),
        );
        menubar.add(
            "File/Print/Question\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| println!("Printing a new Question."),
        );
        menubar.add(
            "File/Print/Variable\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| println!("Printing a new Variable."),
        );
        menubar.add(
            "File/Print/List\t", // Where does versioning come in?
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| println!("Printing a new List."),
        );
        menubar.add(
            "File/Save\t", // Save always focuses on the Question Bank.
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| println!("Saving a Question Bank."),
        );
        menubar.add(
            "File/Save-as\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| println!("Saving a Question Bank with a new name."),
        );

        let quit_idx = menubar.add(
            "File/Quit\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| {
                quit();
            },
        );
        menubar.at(quit_idx).unwrap().set_label_color(Color::Red);
        //endregion

        //region Bank section

        menubar.add(
            "Bank/New\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                bnk_create();
                bnk_refresh_widgets();
            },
        );

        menubar.add(
            "Bank/Open\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                bnk_read();
                bnk_refresh_widgets();
            },
        );

        // TODO: Add  ctrl-s  as option for saving.
        // TODO: Right now  Save  and  Save-As  are the same thing.
        //          Differentiate them.

        menubar.add(
            "Bank/Save\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| {
                bnk_save();
                println!("/n The question bank has been saved. \n")
            },
        );
        menubar.add(
            "Bank/Save-as\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| {
                bnk_save();
                println!("/n The question bank has been saved. \n")
            },
        );

        menubar.add(
            "Bank/Recalculate\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| println!("\n Not yet implemented. \n"),
        );

        menubar.add(
            "Bank/Export\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| println!("\n Not yet implemented. \n"),
        );

        //endregion

        //region Question Section

        menubar.add(
            "Question/New\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                if check_for_bank_loaded() {
                    qst_create();
                    bnk_refresh_widgets();
                }

                // Keep the window display open after this function finishes.
                //  while primwin1.shown() {
                //      app::wait();
                //   }
            },
        );

        menubar.add(
            "Question/Recalculate\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| println!("Not yet implemented.  Recalculating dynamic content in a Question."),
        );

        //endregion

        //region Variable Section

        menubar.add(
            "Variable/New\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                vrbl_create();
            },
        );

        /*
        menubar.add(
            "Variable/New/String\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                vrbl_create("Strings");
            },
        );

        menubar.add(
            "Variable/New/Characters\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                vrbl_create("chars");
            },
        );

        menubar.add(
            "Variable/New/Integers\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                vrbl_create("ints");
            },
        );

        menubar.add(
            "Variable/New/Floats\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                vrbl_create("floats");
            },
        );

        menubar.add(
            "Variable/Recalculate\t",  // Does this make sense as a user task?  Yes, definitely.
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| vrbl_recalc(),
        );

        //endregion

         */
        // Uneeded sub menu items.
        // endregion

        //region List Section

        menubar.add(
            "List/New/Strings\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                list_create("Strings");
            },
        );

        menubar.add(
            "List/New/Characters\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                list_create("chars");
            },
        );

        menubar.add(
            "List/New/Integers\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                list_create("ints");
            },
        );

        menubar.add(
            "List/New/Floats\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                list_create("floats");
            },
        );

        menubar.add(
            "List/Edit\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| println!("\n List editing will be added in a later iteration. \n"),
        );

        // -------  End list section
        //endregion

        menubar
    }
} //  End   menus  module

/// Math-based functions.
///
pub mod math_functions {
    use num_traits::pow;
    use rand::distributions::uniform::SampleUniform;
    use rand::{thread_rng, Rng};

    /// Round an f64 to a given decimal place.
    ///
    pub fn math_round_to_place_f64(num: &f64, place: usize) -> f64 {
        let factor = pow(10, place);
        
        (num * factor as f64).round() / factor as f64
    }

    /// Generate and return a random number between the given min and max.
    ///
    /// Example:
    ///
    ///     fn main() {
    ///         let choice: i64 = math_gen_random_num(-9, 9);
    ///
    ///         println!("\n Your random number is:   {} \n", choice);
    ///     }
    ///
    pub fn math_gen_random_num<T: SampleUniform + PartialOrd>(min: T, max: T) -> T {
        let mut rng = thread_rng();
        rng.gen_range(min..=max)
    }
} // End   math_functions   module

/// Miscellaneous functions used by other modules.
///
pub mod misc {
    use crate::{banks::Bank, questions::qst_edit};
    use crate::{
        Wdgts, CURRENT_BANK, DEVELOPMENT_VERSION, PROGRAM_TITLE, QDISP_HEIGHT, SCROLLBAR_WIDTH,
        VERSION, WIDGETS,
    };
    use fltk::prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt};
    use fltk::text::{TextBuffer, TextDisplay, TextEditor};
    use fltk::{button::Button, enums::Color, group::Scroll};
    use fltk::{text, window::Window};
    use std::fs;

    /// Gets and returns the text from a given FLTK TextEditor.
    ///
    pub fn get_text_from_editor(editor: &TextEditor) -> String {
        if let Some(buffer) = editor.buffer() {
             // Retrieve the text from the associated buffer
            buffer.text()
        } else {
            String::new() // If no buffer is set, return an empty string
        }
    }

    /// Sets up the primary window for QBC.
    ///
    pub fn primwin_setup(primwin: &mut Window) {
        // Set up the primary window.
        //let mut primwin = Window::default().with_size(825, 900).with_pos(1000, 100);
        primwin.set_color(Color::Blue);
        let fulltitle = format!(
            "{} -- {} -- Version {}",
            DEVELOPMENT_VERSION, PROGRAM_TITLE, VERSION
        );
        primwin.set_label(fulltitle.as_str());
        primwin.make_resizable(true);
    }


    /// Creates and sets up an FLTK TextEditor window to use for
    /// a title box when displaying a question Bank.
    pub fn make_title_txtedtr() {
        // todo: Add a line, smaller font, below the main title for
        //      a subtitle or maybe the title of the textbook being used.

        let usebank: Bank;
        let mut wdgts: Wdgts;
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
            wdgts = WIDGETS.lock().unwrap().clone();
        } // Load the global structs.

        let mut buf = TextBuffer::default();
        buf.set_text(usebank.bank_title.as_str()); // Uses the title from the current bank.

        let mut ted = TextEditor::new(0, 40, wdgts.prim_win.width(), 60, "");
        ted.set_text_size(32);
        ted.set_text_color(Color::White);
        ted.set_color(Color::DarkMagenta);
        ted.set_buffer(buf.clone()); // Clone is used here to avoid an ownership error.

        wdgts.title_editbox = ted.clone(); // Store the widgit in the widget struct.
        wdgts.prim_win.add(&ted.clone());

        *WIDGETS.lock().unwrap() = wdgts.clone(); // Update the WIDGET global variable.

        // todo: It would be nice to center and bold the text, but that is really
        //      difficult to do, so leave for later.
    }

    /// Creates and sets up an FLTK scrollgroup to use
    /// when displaying a question Bank.
    pub fn make_scrollgroup() {
        let mut wdgts: Wdgts;
        let usebank: Bank;
        {
            wdgts = WIDGETS.lock().unwrap().clone();
            usebank = CURRENT_BANK.lock().unwrap().clone();
        } // Access the global structs.

        // Create scroll group
        let mut scroll = Scroll::new(
            0,
            wdgts.title_editbox.height() + 40,
            wdgts.prim_win.width(),
            usebank.question_vec.len() as i32 * QDISP_HEIGHT,
            "",
        );
        scroll.set_scrollbar_size(SCROLLBAR_WIDTH);

        // Add scroll to the Wdgts struct & window.
        wdgts.scroll = scroll.clone();
        wdgts.prim_win.add(&scroll.clone());

        *WIDGETS.lock().unwrap() = wdgts.clone(); // Update the WIDGET global variable.
    }

    /// Creates and sets up an FLTK TextDisplay boxes to use
    /// when displaying the questions in a question Bank.
    pub fn make_question_boxes() {
        // region TODO's
        //todo: The question numbers are displaying weird.  First question's label doesn't
        //          even show.  Work on that.  Next iteration.
        // TODO: Set up show/edit prereqs and objectives button
        // TODO: Create a subframe to display/edit the answer.

        // endregion

        let usebank: Bank;
        let mut wdgts: Wdgts;
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
            wdgts = WIDGETS.lock().unwrap().clone();
        } // Access the global structs.

        // region Calculate size and position values.
        let mut box_y = wdgts.title_editbox.height() + 1; // Allow room for the Title Box
        box_y += 60; // Allow room for the label of the first question box.
        let mut qnum = 1; // Question number -- starts at 1.
                          // endregion

        // Set up a display box for each question in the bank.
        for item in usebank.question_vec.iter() {
            // region Create the question label and set up text buffer.
            let qlabel = format!("Question {} :  ", qnum);
            let mut txtbuff = TextBuffer::default();
            txtbuff.set_text(item.qtext.as_str());
            // endregion

            // region Setup the display box and it's attributes.
            let mut quest_disp = TextDisplay::new(
                0,
                box_y,
                wdgts.scroll.w() - SCROLLBAR_WIDTH,
                QDISP_HEIGHT,
                qlabel.as_str(),
            );
            quest_disp.set_buffer(txtbuff);
            quest_disp.wrap_mode(text::WrapMode::AtBounds, 0);
            quest_disp.set_color(fltk::enums::Color::White);
            quest_disp.set_text_size(22);
            quest_disp.set_text_color(fltk::enums::Color::Black);
            // endregion

            // region Setup the edit button & callback. Buttons not added to widget struct.
            let editbtn_x = quest_disp.x() + quest_disp.w() - 65; // Button is sized 50 X 30
            let editbtn_y = quest_disp.y() + quest_disp.h() - 35;
            let mut editbtn = Button::new(editbtn_x, editbtn_y, 50, 30, "Edit");

            editbtn.set_callback(move |_| {
                println!("\n Edit button for Question #{} has been pressed. \n", qnum);
                qst_edit(qnum - 1);
            });
            // endregion

            // region Increment values and push/add items to WIDGETS struct
            box_y += QDISP_HEIGHT; // Increment the question display widget position.
            qnum += 1; // Increment the question display number.

            wdgts.qstn_boxes.push(quest_disp.clone());
            wdgts.scroll.add(&quest_disp);
            wdgts.scroll.add(&editbtn);
            // endregion
        }
        *WIDGETS.lock().unwrap() = wdgts.clone(); // Update the WIDGET global variable.
    }

    /// Check to see whether or not a bank has been loaded into memory.
    ///
    pub fn check_for_bank_loaded() -> bool {
        let usebank = CURRENT_BANK.lock().unwrap().clone();
        if usebank.bank_title.is_empty() {
            println!("\n No bank has been loaded. \n"); // todo: Find a non-terminal way to display this.
            println!(" Please open a bank. \n");
            false
        } else {
            true
        }
    }

    /// Gets an FLTK window's position and dimension attributes.
    /// Here for debugging purposes only.
    pub fn get_window_attrs(win: &Window) {
        // For debugging purposes.  Move to  lib.utils???

        let xxx = win.x();
        let yyy = win.y();
        let www = win.w();
        let hhh = win.h();

        println!(
            "\n The position of the primary window is :  ({}, {}) \n",
            xxx, yyy
        );
        println!("The size of the primary window is :  ({}, {}) \n", www, hhh);
    }

    /// EVENTUALLY MOVE THIS FUNCTION TO  `lib_file`
    ///
    /// Note that this function is not safe as it does not check for errors.
    /// Be sure that the `path` parameter has been validated
    /// before calling this function.
    ///
    /// #Example
    ///
    ///     fn main() {
    ///         let usepath = "/home/jtreagan/programming/mine/cards";
    ///
    ///         if dir_is_empty(usepath) {
    ///             println!("\n The path  {}  is empty \n", usepath);
    ///         } else {
    ///             println!("\n The path  {}  is not empty \n", usepath);
    ///         }
    ///     }
    ///
    pub fn dir_is_empty(path: &str) -> bool {
        fs::read_dir(path)
            .map(|entries| entries.count() == 0)
            .unwrap_or(false)
    }
} // End   misc   module
