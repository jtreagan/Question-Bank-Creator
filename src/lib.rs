
/*
        // TODO: Font Libre Baskerville, 14 pt.  -- next iteration.
        // TODO: All this needs to be user modifiable. -- next iteration.
        // TODO: Goal is for  WYSIWIG.  --  in the far, far future.

        // TODO: Add second line to the title containing the associated textbook text.
        // TODO: Question display should show calculated values for the variables
        //          rather than the variable ID.  Maybe highlight the values so
        //          that the variable can be easily located.

 */  // TODO's

use crate::banks::Bank;
use fltk::app::App;
use fltk::group::Scroll;
use fltk::prelude::{GroupExt, WidgetExt};
use fltk::text::{TextDisplay, TextEditor};
use fltk::utils::oncelock::Lazy;
use fltk::window::Window;
use std::sync::Mutex;

// region  Global Constants

/// The current iteration of the program being worked on.
pub const DEVELOPMENT_VERSION: &str = "Question Bank Rebuild 4";
/// The title of the project.
pub const PROGRAM_TITLE: &str = "Question Bank Creator";
/// The current version..
pub const VERSION: &str = "0.29.7";     // Note:  Versioning is decimal in nature.

/// The default folder where data is saved.
pub const DATA_GENERAL_FOLDER: &str = "/home/jtreagan/programming/rust/mine/qbnk_data";
/// The default folder for saving Lists.
pub const LIST_DIR: &str = "/home/jtreagan/programming/rust/mine/qbnk_data/lists";
/// The default folder for saving Variables.
pub const VARIABLE_DIR: &str = "/home/jtreagan/programming/rust/mine/qbnk_data/variables";
/// The default folder for saving Banks.
pub const BANK_DIR: &str = "/home/jtreagan/programming/rust/mine/qbnk_data/banks";
/// Default height of the question display.
pub const QDISP_HEIGHT: i32 = 150;
/// Default width of the scrollbar.
pub const SCROLLBAR_WIDTH: i32 = 15;
// endregion

//region Global Variables
/// Contains the question Bank that is currently being edited.
pub static CURRENT_BANK: Lazy<Mutex<Bank>> = Lazy::new(|| Mutex::new(Bank::new()));
/// Contains the last directory path that was used.
pub static LAST_DIR_USED: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
/// Holds the currently running FLTK App.
pub static APP_FLTK: Lazy<Mutex<App>> = Lazy::new(|| Mutex::new(App::default()));
/// Holds the FLTK widgets currently being used.
pub static WIDGETS: Lazy<Mutex<Wdgts>> = Lazy::new(|| Mutex::new(Wdgts::new()));
//endregion

// region structs
/// Struct that holds the primary window's widgets.
///
pub struct Wdgts {
    pub prim_win: Window,
    pub title_editbox: TextEditor,
    pub scroll: Scroll,
    pub qstn_boxes: Vec<TextDisplay>,
}

impl Wdgts {
    pub fn new() -> Self {
        let prim_win = Window::default()
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

// endregion

/// Holds the TypeWrapper enum.
///
pub mod global {
    // todo:  Do you really need this?

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum TypeWrapper {
        Alphanum(String),
        Letter(char),
        Integer(i64),
        Floating(f64),
    }
}  // End   global   module

/// Functions that deal with the Bank struct.
///
pub mod banks {
    use crate::misc::{make_question_boxes, make_scrollgroup, make_title_txtedtr};
    use crate::{questions::*, Wdgts, APP_FLTK, BANK_DIR, CURRENT_BANK, LAST_DIR_USED, WIDGETS};
    use fltk::prelude::{DisplayExt, GroupExt, WidgetExt};
    use fltk::text::TextBuffer;
    use fltk::app;
    use lib_file::{file_fltk::*, file_mngmnt::file_read_to_string};
    use lib_myfltk::{fltkutils::*, input_fltk::*};
    use serde::{Deserialize, Serialize};
    use std::{fs::File, io::Write};

    //region Struct Section
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Bank {
        pub bank_title: String,   // Also used for file name.
        pub associated_textbook: String,   // Use  ""  if no text being used.
        pub question_vec: Vec<Question>,
    }

    impl Bank {
        pub fn new() -> Bank {
            Self {
                bank_title: "No Bank Loaded".to_string(),
                associated_textbook: "Untitled Textbook".to_string(),
                question_vec: Vec::new(),
            }
        }
    }

    impl Clone for Bank {
        fn clone(&self) -> Self {
            Self {
                bank_title: self.bank_title.clone(),
                associated_textbook: self.associated_textbook.clone(),
                question_vec: self.question_vec.clone(), // Vec itself implements Clone.
            }
        }
    }

    //endregion

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
        let mut app = app::App::default();
        {
            app = APP_FLTK.lock().unwrap().clone();
        } // Access the main app.


        //let usetitle =

        // Input values into the struct fields.
        let mut newbank = Bank::new();
        newbank.bank_title = input_string(&app, "Please enter the bank's title.", 300, 90);
        newbank.associated_textbook = input_string(&app, "If you are using an associated textbook \n please enter its info. \n Press  Enter  if no textbook is being used.",
                                                   800, 200);
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

        // region Set up directories.

                    // Setup proper directory and read the file.
        println!("\n Please choose the Bank file to be read.");

        let usepath: String;

        { // Global variable scope is restricted to avoid Mutex lock.
            if LAST_DIR_USED.lock().unwrap().clone() == "" {
                *LAST_DIR_USED.lock().unwrap() = BANK_DIR.to_string().clone();
            }
            let lastdir = LAST_DIR_USED.lock().unwrap().clone();
            usepath = file_fullpath(&lastdir);
            *LAST_DIR_USED.lock().unwrap() = usepath.clone();  // Update LAST_DIR_USED
        }
        //endregion

        // region Read the chosen file.
        let usebank: Bank;
        match file_read_to_string(&usepath) {
            Ok(contents) => {
                usebank = serde_json::from_str(&contents).unwrap();
                *CURRENT_BANK.lock().unwrap() = usebank.clone();
            }
            // TODO: Fix error handling.  This is terrible.  See thread in forum at
            // https://users.rust-lang.org/t/help-understanding-never-used-warning/125562/2
            Err(err) => {
                eprintln!("\n Error reading the file: {} \n", err);
                panic!("\n Error reading the file. \n");
            }
        }
        // endregion

        {
            *CURRENT_BANK.lock().unwrap() = usebank.clone();
        }  // Pass the new bank into CURRENT_BANK
    }

    /// Refreshes the contents of the title box of a bank's display.
    ///
    pub fn bnk_refresh_title() {
        let usebank: Bank;
        let mut wdgts: Wdgts;
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
            wdgts = WIDGETS.lock().unwrap().clone();
        }  // Access global structs.

        let mut buf = TextBuffer::default();
        buf.set_text(usebank.bank_title.as_str());  // Uses the title from the current bank.
        wdgts.title_editbox.set_buffer(buf);

        //  let title_text =   // There is likely to be a use for  title_text   in the future.
        wdgts.title_editbox.buffer().unwrap().text();
    }

    /// Prepares a Bank struct for saving.
    ///
    pub fn bnk_save() {
        // region TODO's
        // TODO: Find way to insert bank title into the save-file dialog.
        // TODO: Find way to append correct extension automatically.
        // endregion

        if LAST_DIR_USED.lock().unwrap().clone() == "" {
            *LAST_DIR_USED.lock().unwrap() = BANK_DIR.to_string().clone();
        }  // If no bank loaded, use default.

        let lastdir: String;
        {
            lastdir = LAST_DIR_USED.lock().unwrap().clone();
        }

        println!("Please choose a directory and file name for saving. \n");
        let usepath = file_browse_save_fltr(&lastdir, "*.bnk");

        {
            *LAST_DIR_USED.lock().unwrap() = usepath.clone();
        }  // Set the last used directory.

        bnk_save_as_json(&usepath);
    }

    /// Saves a Bank struct to a file in json format.
    ///
    pub fn bnk_save_as_json(usepath: &String) {

        let mut usebank = Bank::new();
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
        }

        let bnk_as_json = serde_json::to_string(&usebank).unwrap();  // Convert bank to json string.

        let mut file = File::create(usepath).expect("Could not create file!");

        file.write_all(bnk_as_json.as_bytes())
            .expect("Cannot write to the file!");
    }

    /// Recalculates the variables in the questions of a Bank.
    ///
    pub fn bnk_recalc() {
        let mut usebank = Bank::new();
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

    /*
    pub fn test_globalbank_access() {
        let testbank = CURRENT_BANK.lock().unwrap().clone();
        println!("\n The test bank is: \n {:?} \n", testbank);
    }


    pub fn temp_listwindows(app: &App) {

    // Retrieve all open child windows
    let windows = app::windows();
    println!("\n Currently Open Child Windows: \n");

    for item in windows.iter() {
        let winlabel = item.label();
        println!("\n Window: {} \n", winlabel);
    }
}
*/  // delete later

}  // End    bank    module

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
    use lib_file::file_mngmnt::{file_get_dir_list, file_read_to_string};
    use lib_myfltk::fltkutils::*;
    use lib_myfltk::input_fltk::{input_string, input_strvec};
    use lib_utils::utilities::*;
    use serde::{Deserialize, Serialize};

    //region Struct Section

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
    }  // End   Question   impl
    //endregion

    pub fn qst_create() {
    // todo: The answer will need to parse inserted variables.

        let mut newquest = Question::new();

// region Question data entry

        let nowtext = qst_editor(newquest.qtext.as_str(), "Question Editor");
        newquest.qtext = nowtext.clone();

        // Pull the flagged variables from the text and push them to the variable vector.
        qst_fill_varvec_parsetext(&mut newquest);

                // Answer will eventually need to be calculated.

        let mut app = app::App::default();
        {
            app = APP_FLTK.lock().unwrap().clone();
        }

        newquest.answer = input_string(&app, "Please input the question's answer:  ", 790, 300);
        newquest.objectives = input_strvec(&app, "Please enter the question objectives:  ", 790, 300);
        newquest.prereqs = input_strvec(&app, "Please enter the question prerequisites:  ", 790, 300);
// endregion

// region Save and store the data
        let mut usebank: Bank;
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
        }  // Access the global Bank variable
        usebank.question_vec.push(newquest);  // Store the new question in the bank
        {  // Pass the modified bank into the global variable.
            *CURRENT_BANK.lock().unwrap() = usebank.clone();
        }
        bnk_save();
// endregion

    }

    pub fn qst_edit(qst_idx: usize) {

        let mut app = app::App::default();
        let mut usebank = Bank::new();
        {
            app = APP_FLTK.lock().unwrap().clone();
            usebank = CURRENT_BANK.lock().unwrap().clone();
        }  // Access global variables.

        let mut editqst = usebank.question_vec.get(qst_idx).unwrap().clone();

        let nowtext = qst_editor(editqst.qtext.as_str(), "Question Editor");
        editqst.qtext = nowtext.clone();

        // Pull the flagged variables from the text and push them to the variable vector.
        qst_fill_varvec_parsetext(&mut editqst);  // Need to clear the vector first.

        // Answer will eventually need to be calculated.

        editqst.answer = input_string(&app,"Please input the question's answer:  ", 790, 300);
        editqst.objectives = input_strvec(&app,"Please enter the question objectives:  ", 790, 300);
        editqst.prereqs = input_strvec(&app,"Please enter the question prerequisites:  ", 790, 300);

        // Push the question to the vector in the bank and save the bank.
        //let mut usebank = CURRENT_BANK.lock().unwrap();

        // todo: This won't work.  push()  appends to the end of the vector. Fix it.
        usebank.question_vec.push(editqst.clone());
        bnk_save();
        bnk_refresh_widgets();
    }

    pub fn qst_chooseqst() -> Question {

        // TODO: Instead of trying to put the whole text of the question
        //          body in the radio button, number each question in the
        //          bank display and choose by the question number.

        // Note:  This function may not be necessary.

        let mut usevec: Vec<String> = Vec::new();

        let mut usebank = Bank::new();
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
        }

        for item in usebank.question_vec.iter() {
            usevec.push(item.qtext.clone());
        }

        let usequest = fltk_radio_lightbtn_menu(&usevec);
        let mut editquest = Question::new();

        for item in usebank.question_vec.iter() {
            if item.qtext == usequest {
                editquest = item.clone();
            }
        }

        editquest
    }  // Is this necessary now?

    pub fn qst_fill_varvec_parsetext(quest: &mut Question) {
        // region Create a vector of the variable names that have been flagged in the text.
        let mut usevec = util_flaggedtxt_2vec(&quest.qtext, '§');
        usevec.sort();
        usevec.dedup();     // Remove repeats of the flagged variable names.
        // endregion

        // region Read the variable files from disk and insert them into the variable vector.
        quest.var_vec.clear();
        for _item in usevec {
            let newvar = vrbl_read();
            quest.var_vec.push(newvar);
        }
        // endregion
    }

    /*
        pub fn qst_fill_varvec_dirlist() -> Vec<Variable> {
            println!("\n Please choose the variables you want to include as part of your question:  ");
            let path = file_pathonly();
            let flist = file_get_dir_list(&path);
            let flist_vec = chkbox_shift_menu(&flist);
            let mut usevec: Vec<Variable> = Vec::new();

            for item in flist_vec {
                let flist_fullpath = format!("{}/{}", path, item);
                println!("{}", flist_fullpath);
                usevec.push(vrbl_read_nogetpath(&flist_fullpath));
            };
            usevec
        }
    */   //fn qst_fill_varvec_dirlist()

    pub fn qst_editor(startertxt: &str, winlabel: &str) -> String {

        let mut buf = TextBuffer::default();
        let mut edtrwin = window::Window::default().with_size(800, 300);
        set_font_size(20);
        edtrwin.set_color(Color::Blue);
        edtrwin.set_label(winlabel);
        edtrwin.make_resizable(true);

        buf.set_text(startertxt);
        let mut edtr = TextEditor::default()
            .with_size(770, 222)
            .center_of_parent();

        qst_editor_menubar(&edtr, &mut edtrwin, &mut buf);

        edtr.set_buffer(buf.clone());   // Clone is used here to avoid an ownership error.
        edtr.wrap_mode(text::WrapMode::AtBounds, 0);
        edtr.set_color(Color::White);
        edtr.set_text_size(22);
        edtr.set_text_color(Color::Black);

        edtrwin.end();
        edtrwin.show();

        while edtrwin.shown() {
            app::wait();
        }

        println!("\n W5:  End of qst_editor().  The quesion text is:  {} \n", buf.text());

        buf.text()
    }

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
                let newtext  = qst_make_var_replace_text();
                fltk_replace_highlighted_text(&edtr_clone, &mut buf_clone, &newtext);
            },
        );
        // endregion

        menubar
    }

    pub fn qst_make_var_replace_text() -> String {
        let usedir = VARIABLE_DIR.to_string();

        println!("Please choose the variable you want to insert. \n");

        println!("\n W1: Start of qst_make_var_replace_text() \n");
        let path = file_pathonly(&usedir);
        println!("\n W2: path = {} \n", path);

        let flist = file_get_dir_list(&path);
        println!("\n W3: flist = {:?} \n", flist);

        let varname = fltk_radio_lightbtn_menu(&flist);
        println!("\n W4: varname = {} \n", varname);

        let rpltxt = format!("§{}§", varname);

        rpltxt
    }

    pub fn qst_read() -> Question {

        // region Choose the desired path.
        let mut usedir = String::new();
        {
            usedir = LAST_DIR_USED.lock().unwrap().clone();
        }
        println!("\n Please choose the Question file to be read.");
        let usepath = file_fullpath(&usedir);
        // endregion

        match file_read_to_string(&usepath) {
            Ok(contents) => {
                let newquest = serde_json::from_str(&contents).unwrap();
                newquest
            }
            Err(err) => {
                eprintln!("\n Error reading the file: {} \n", err);
                panic!("\n Error reading the file. \n");
            }
        }

    }

    /*

        --How are you going to insert these variables into the text of the question?
            -- You will have to use the cursor position when the user selects
                the text being replaced by the variable.  That will be your break point.
            -- Remember, you are entering the variable data into the question vector,
                not the variable itself.
                -- Only that's not true.  You need the variable itself there, somehow
                    to use for recalculating the dynamic values.

        -- Answers will be calculated from the current variable values.

        -- What are you going to do about operators and how they interact, especially
            when the operator is given in the question in verbal format?
            -- The answer equation will have to be entered by the user.

        -- And then there be equations!!!!

     */  // Issues & questions

}  // End   questions   module

/// Functions that deal with the Variable struct.
///
pub mod variable {
    use crate::global::TypeWrapper;
    use crate::global::TypeWrapper::*;
    use crate::lists::list_read;
    use crate::math_functions::*;
    use crate::{LAST_DIR_USED, VARIABLE_DIR};
    use lib_file::file_fltk::*;
    use lib_file::file_mngmnt::{file_path_to_fname, file_read_to_string};
    use lib_utils::{input_utilities::*, vec::*};
    use serde::{Deserialize, Serialize};
    use std::{fs::File, io::Write};

    //region Struct Section
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Variable {
        pub var_fname: String,
        pub params: VarPrmtrs,
        pub list_fname: String,
        pub content: TypeWrapper,
        pub var_type: String,
    }

    impl Variable {
        pub fn new() -> Variable {
            Self {
                var_fname: "new_variable".to_string(),
                params: VarPrmtrs::new(),
                list_fname: "".to_string(),
                content: Integer(0),
                var_type: "Strings".to_string(),   // "Strings", "chars", "ints", "floats"
            }
        }
    } // End Variable impl

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct VarPrmtrs {
        pub is_string: bool,
        pub is_char: bool,
        pub is_from_list: bool,
        pub is_int: bool,
        pub num_min_int: i64,
        pub num_max_int: i64,
        pub num_min_float: f64,
        pub num_max_float: f64,
        pub num_dcml_places: usize,
        pub num_comma_frmttd: bool,
    }

    impl VarPrmtrs {
        pub fn new() -> VarPrmtrs {
            Self {
                is_string: false,
                is_char: false,
                is_from_list: false,
                is_int: true,
                num_min_int: 0,
                num_max_int: 0,
                num_min_float: 0.0,
                num_max_float: 0.0,
                num_dcml_places: 0,
                num_comma_frmttd: false,  // Leave implementing this until you need to output it.

                // Default values all assume that the variable is an i64.
            }
        }
    } // ~~~~~ End VarPrmtrs impl ~~~~~
    //endregion

    pub fn vrbl_create(typch: &str) {
        let mut var1 = Variable::new();
        var1.var_type = typch.to_string();
        vrbl_input_parameters(&mut var1);
        vrbl_input_vardata(&mut var1);
        vrbl_save(&mut var1);
    }

    pub fn vrbl_input_parameters(data: &mut Variable) {  // Set boolean parameters only.  Leave data alone.

        // todo: Turn all this into a window of radio and checkbox buttons for setting
        //          these parameters.

        match data.var_type.as_str() {
            "Strings" => {  // Note that Strings should only come from a list.
                data.params.is_string = true;
                data.params.is_int = false;
                data.params.is_from_list = true;
            }

            "chars" => {     // Note that characters should only come from a list.
                data.params.is_char = true;
                data.params.is_int = false;
                data.params.is_from_list = true;
            }

            "ints" => {
                data.params.num_comma_frmttd = input_bool_prompt("\n Is the value to be comma formatted?   ");

                let mini_choice = input_bool_prompt("\n Is the variable contents to come from a list?   ");
                if mini_choice {
                    data.params.is_from_list = true;
                    return;
                }
                data.params.num_min_int = input_num_prompt("\n Please enter the minimum int value:  ");
                data.params.num_max_int = input_num_prompt("\n Please enter the maximum int value:  ");
            }

            "floats" => {
                data.params.is_int = false;
                data.params.num_dcml_places = input_num_prompt("\n How many decimal places are allowed?  ");
                data.params.num_comma_frmttd = input_bool_prompt("\n Is the value to be comma formatted?   ");

                let mini_choice = input_bool_prompt("\n Is the variable contents to come from a list?   ");
                if mini_choice {
                    data.params.is_from_list = true;
                    return;
                }
                data.params.num_min_float = input_num_prompt("\n Please enter the minimum float value:  ");
                data.params.num_max_float = input_num_prompt("\n Please enter the maximum float value:  ");
            }

            _ => { unreachable!(); }
        }
    }

    pub fn vrbl_input_vardata(data: &mut Variable) {
        data.var_fname = input_string_prompt("\n Please enter a title/filename for your new variable:  ");
        vrbl_setvalues(data);
    }

    pub fn vrbl_save(var1: &mut Variable) {
        let lastdir = String::new();
        {
            if LAST_DIR_USED.lock().unwrap().clone() == "" {
                *LAST_DIR_USED.lock().unwrap() = VARIABLE_DIR.to_string().clone();
            }  // If no path loaded, use default.
            let lastdir = LAST_DIR_USED.lock().unwrap().clone();
        }  // Access the global variable.

        let usepath = file_browse_save_fltr(&lastdir, "Variable Files   \t*.vrbl\nText Files   \t*.txt\nList Files    \t*.lst\nAll Files    \t*.*");

        {
            *LAST_DIR_USED.lock().unwrap() = usepath.clone();
        }  // Set LAST_DIR_USED to the new path.

        var1.var_fname = file_path_to_fname(&usepath);
        vrbl_save_as_json(&var1, &usepath);

        println!("\n The variable has been saved.");
    }

    pub fn vrbl_save_as_json(var: &Variable, usepath: &String) {
        let var_as_json = serde_json::to_string(var).unwrap();

        let mut file = File::create(usepath.as_str()).expect("Could not create file!");

        file.write_all(var_as_json.as_bytes())
            .expect("Cannot write to the file!");
    }

    pub fn vrbl_read() -> Variable {

        // region Choose the correct directory path
        let mut usepath = VARIABLE_DIR.to_string();
        println!("\n Please choose the variable file to be used.");
        usepath = file_fullpath(&usepath);
        {
            *LAST_DIR_USED.lock().unwrap() = usepath.clone();
        }  // Set LAST_DIR_USED to the new path.
        // endregion

        match file_read_to_string(&usepath) {
            Ok(contents) => {
                let newvariable = serde_json::from_str(&contents).unwrap();
                newvariable
            }
            Err(err) => {
                eprintln!("\n Error reading the file: {} \n", err);
                panic!("\n Error reading the file. \n");
            }
        }
    }

    /*
    pub fn vrbl_read_nogetpath(usepath: &Rc<RefCell<String>>) -> Variable {
        // Should return an option or result rather than  `unwrap()`.

        let data = util_read_file_to_string(&usepath);
        let newvrbl = serde_json::from_str(&data).unwrap();

        newvrbl
    }

     */   // vrbl_read_nogetpath()

    pub fn vrbl_setvalues(var1: &mut Variable) {  // Note that this deals with
        //let lastdir = LAST_DIR_USED.lock().unwrap();

        if var1.params.is_from_list {  // The variable content is to come from a list.
            match var1.var_type.as_str() {
                "Strings" => {
                    println!("\n Please choose the list you want to use.");
                    let newlist = list_read("Strings");  // Returns a tuple (listname, List)
                    var1.list_fname = newlist.0;  // Sets the value of the variable's listname field.

                    let usevec = newlist.1.words.clone();  // Clones the list content vector.
                    //let usevec_str = vec_string_to_str(&usevec);
                    let item = vec_random_choice(&usevec);
                    match item {
                        Some(x) => {
                            println!("\n The chosen item is:  {:?}", x);
                            var1.content = Alphanum(x.0.to_string());
                        },
                        None => panic!("No item was chosen."),
                    }
                },

                "chars" => {
                    println!("\n Please choose a list to be read.");
                    let newlist = list_read("chars");
                    var1.list_fname = newlist.0;

                    let item = vec_random_choice(&newlist.1.runes);
                    match item {
                        Some(x) => {
                            println!("\n The chosen item is:  {:?}", x);
                            var1.content = Letter(*x.0);
                        },
                        None => panic!("No item was chosen."),
                    }
                },

                "ints" => {
                    println!("\n Please choose a list to be read.");
                    let newlist = list_read("ints");
                    var1.list_fname = newlist.0;

                    let item = vec_random_choice(&newlist.1.intsigned);
                    match item {
                        Some(x) => {
                            println!("\n The chosen item is:  {:?}", x);
                            var1.content = Integer(*x.0);
                        },
                        None => panic!("No item was chosen."),
                    }
                },

                "floats" => {
                    println!("\n Please choose a list to be read.");
                    let newlist = list_read("floats");
                    var1.list_fname = newlist.0;

                    let item = vec_random_choice(&newlist.1.decimals);
                    match item {
                        Some(x) => {
                            println!("\n The chosen item is:  {:?}", x);
                            var1.content = Floating(*x.0);
                        },
                        None => panic!("No item was chosen."),
                    }
                },

                _ => {}
            }
        } else {
            if var1.params.is_int {  // Numeric values will always be randomly generated.
                let numint: i64 = math_gen_random_num(var1.params.num_min_int, var1.params.num_max_int);
                var1.content = Integer(numint);
            } else {  // The content is a float.
                let mut numfloat: f64 = math_gen_random_num(var1.params.num_min_float, var1.params.num_max_float);
                numfloat = math_round_to_place_f64(&numfloat, var1.params.num_dcml_places);
                var1.content = Floating(numfloat);
            }
        }
    }

    pub fn vrbl_recalc() {
        let mut usevar = vrbl_read();

        println!("\n The variable before recalc is: \n {:?}", usevar);

        vrbl_setvalues(&mut usevar);
        vrbl_save(&mut usevar);

        println!("\n The variable after recalc is: \n {:?} \n", usevar);

    }
} // End   variable   module

/// Functions for creating and manipulating lists.
///
pub mod lists {
    use crate::{APP_FLTK, LAST_DIR_USED};
    use fltk::app::App;
    use lib_file::file_fltk::{file_browse_save_fltr, file_fullpath_fltr};
    use lib_file::file_mngmnt::file_read_to_string;
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::Write;
    use lib_myfltk::input_fltk::*;

    // region Struct section
    #[derive(Debug, Serialize, Deserialize)]
    pub struct List {
        pub words: Vec<String>,
        pub runes: Vec<char>,
        pub intsigned: Vec<i64>,
        pub decimals: Vec<f64>,
        pub typechoice: String, // "Strings", "chars", "ints", "floats"
    }

    impl List {
        pub fn new() -> List {
            Self {
                words: Vec::new(),
                runes: Vec::new(),
                intsigned: Vec::new(),
                decimals: Vec::new(),
                typechoice: "Strings".to_string(),
            }
        }
    }  // ----------  End List impl ----------

    // endregion

    pub fn list_create(typech: &str) {
        let mut app = App::default();
        {
            app = APP_FLTK.lock().unwrap().clone();
        }

        let mut newlist = List::new();
        newlist.typechoice = typech.to_string();

        match typech {
            "String" | "Strings" => {     // String
                let uselist = input_strvec(&app, "Please enter a string.", 790, 300);
                newlist.words = uselist.clone();
                list_save(&newlist);
            }

            "char" | "chars" => {     // char
                let uselist = input_charvec(&app, "Please enter a character." );
                newlist.runes = uselist.clone();
                list_save(&newlist);
            }

            "int" | "ints" => {     // i64
                let uselist = input_i64vec(&app, "Please enter an integer." );
                newlist.intsigned = uselist.clone();
                list_save(&newlist);
            }

            "float" | "floats" => {     // f64

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

    pub fn list_read(typech: &str) -> (String, List) {

        let mut startdir = String::new();
        {
            startdir = LAST_DIR_USED.lock().unwrap().clone();
        }

        // TODO: Should return an option or result rather than  `unwrap()` or `panic!()`.
        // TODO: This has not been tested after the last modifications were made.

        let readlist = loop {
            let usename = file_fullpath_fltr(&startdir, "*.lst");
            *LAST_DIR_USED.lock().unwrap() = usename.clone();

            match file_read_to_string(&usename) {
                Ok(contents) => {
                    let newlist = serde_json::from_str(&contents).unwrap();
                    let typchk = list_check_typematch(&newlist, typech);
                    if !typchk { continue }
                    else { break (usename, newlist)}
                }
                Err(err) => {
                    eprintln!("\n Error reading the file: {} \n", err);
                    panic!("\n Error reading the file. \n");
                }
            }
        };

        readlist
    }
    // Function returns tuple containing the file name that was read
    //      along with the reconstituted list.

    pub fn list_edit() {

        println!("\n Someday I'll write this function. \n");
    }    // This function to be finished in future iteration.

    pub fn list_save(list: &List) -> String {

        let startdir = LAST_DIR_USED.lock().unwrap().clone(); // Access the last used directory.

        let path = file_browse_save_fltr(&startdir, "List Files    \t*.lst\nVariable Files   \t*.vrbl\nText Files   \t*.txt\nAll Files");
        *LAST_DIR_USED.lock().unwrap() = path.clone();  // Store the current path in global.

        list_save_as_json(&list, &path);

        path
    }

    pub fn list_save_as_json(list: &List, fname: &str) {
        let list_as_json = serde_json::to_string(list).unwrap();

        let mut file = File::create(fname).expect("Could not create file!");

        file.write_all(list_as_json.as_bytes())
            .expect("Cannot write to the file!");
    }

    pub fn list_check_typematch(uselist: &List, typech: &str) -> bool {
        if uselist.typechoice.as_str() != typech {
            println!("\n The data type of that list does not match your typechoice. \n");
            println!("Please choose a different list file. \n");
            false
        } else { true }
    }



}  // End  lists module

/// Functions for use in creating menus.
///
pub mod menus {
    use crate::{banks::*, questions::*, variable::*, lists::*};
    use fltk::app::quit;
    use fltk::enums::{Color, Shortcut};
    use fltk::menu;
    use fltk::prelude::{MenuExt, WidgetBase, WidgetExt};
    use fltk::window::Window;
    use crate::misc::check_for_bank_loaded;

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
            "File/Print/List\t",  // Where does versioning come in?
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
            },        );


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

    pub fn math_round_to_place_f64(num: &f64, place: usize) -> f64 {
        let factor = pow(10, place);
        let rounded = (num * factor as f64).round() / factor as f64;
        return rounded;
    }

    pub fn math_gen_random_num<T: SampleUniform + PartialOrd>(min: T, max: T) -> T {
        let mut rng = thread_rng();
        rng.gen_range(min..=max)
    }

    /*
    fn main() {
        let choice: i64 = math_gen_random_num(-9, 9);

        println!("\n Your random number is:   {} \n", choice);
    }

 */ // Example usage math_gen_random_num()


} // End   math_functions   module.

/// Miscellaneous functions used by other modules.
///
pub mod misc {
    use crate::banks::Bank;
    use crate::questions::qst_edit;
    use crate::{Wdgts, CURRENT_BANK, DEVELOPMENT_VERSION, PROGRAM_TITLE, QDISP_HEIGHT, SCROLLBAR_WIDTH, VERSION, WIDGETS};
    use fltk::prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt};
    use fltk::text;
    use fltk::text::{TextBuffer, TextDisplay, TextEditor};
    use fltk::window::Window;
    use fltk::{button::Button, enums::Color, group::Scroll};

    pub fn get_text_from_editor(editor: &TextEditor) -> String {
        if let Some(buffer) = editor.buffer() {
            let text = buffer.text(); // Retrieve the text from the associated buffer

            println!("\n The text from the editor is: {} \n", buffer.text());

            return text;

        } else {
            String::new() // If no buffer is set, return an empty string
        }
    }

    pub fn primwin_setup(primwin: &mut Window) {  // Set up the primary window.
        //let mut primwin = Window::default().with_size(825, 900).with_pos(1000, 100);
        primwin.set_color(Color::Blue);
        let fulltitle = format!("{} -- {} -- Version {}", DEVELOPMENT_VERSION, PROGRAM_TITLE, VERSION);
        primwin.set_label(fulltitle.as_str());
        primwin.make_resizable(true);
    }

    /*
    pub fn onopen_popup() -> Window {
        // On program opening, pop up a window with choice for new bank or open existing bank.

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

        let mut wdgts = Wdgts::new();
        {
            wdgts = WIDGETS.lock().unwrap().clone();
        }  // Access the WIDGETS struct

        // todo: The function below is way too complex.  Find another solution.
        let mut pop = fltk_popup_2btn(&wdgts.prim_win, Box::new(bttn_newbank), "Create new bank",
                                  Box::new(bttn_openbank), "Open existing bank");
        pop.set_color(Color::Red);

        pop
    }

     */  // No longer needed -- onopen_popup()

    pub fn make_title_txtedtr() {

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

        let mut ted = TextEditor::new(0, 40, wdgts.prim_win.width(), 60, "");
        ted.set_text_size(32);
        ted.set_text_color(fltk::enums::Color::White);
        ted.set_color(fltk::enums::Color::DarkMagenta);
        ted.set_buffer(buf.clone());   // Clone is used here to avoid an ownership error.

        wdgts.title_editbox = ted.clone();  // Store the widgit in the widget struct.
        wdgts.prim_win.add(&ted.clone());

        *WIDGETS.lock().unwrap() = wdgts.clone();    // Update the WIDGET global variable.

        // todo: It would be nice to center and bold the text, but that is really
        //      difficult to do, so leave for later.
    }

    pub fn make_scrollgroup() {

        let mut wdgts: Wdgts;
        let usebank: Bank;
        {
            wdgts = WIDGETS.lock().unwrap().clone();
            usebank = CURRENT_BANK.lock().unwrap().clone();
        }  // Access the global structs.

        // Create scroll group
        let mut scroll = Scroll::new(0, wdgts.title_editbox.height() + 40,
                                     wdgts.prim_win.width(),
                                     usebank.question_vec.len() as i32 * QDISP_HEIGHT,
                                     "");
        scroll.set_scrollbar_size(SCROLLBAR_WIDTH);

        // Add scroll to the Wdgts struct & window.
        wdgts.scroll = scroll.clone();
        wdgts.prim_win.add(&scroll.clone());

        *WIDGETS.lock().unwrap() = wdgts.clone();    // Update the WIDGET global variable.
    }

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
        }  // Access the global structs.

        // region Calculate size and position values.
        let mut box_y = wdgts.title_editbox.height() + 1;   // Allow room for the Title Box
        box_y += 60;            // Allow room for the label of the first question box.
        let mut qnum = 1;       // Question number -- starts at 1.
        // endregion

        // Set up a display box for each question in the bank.
        for item in usebank.question_vec.iter() {

            // region Create the question label and set up text buffer.
            let qlabel = format!("Question {} :  ", qnum);
            let mut txtbuff = TextBuffer::default();
            txtbuff.set_text(item.qtext.as_str());
            // endregion

            // region Setup the display box and it's attributes.
            let mut quest_disp = TextDisplay::new(0,
                                                  box_y,
                                                  wdgts.scroll.w() - SCROLLBAR_WIDTH,
                                                  QDISP_HEIGHT,
                                                  qlabel.as_str());
            quest_disp.set_buffer(txtbuff);
            quest_disp.wrap_mode(text::WrapMode::AtBounds, 0);
            quest_disp.set_color(fltk::enums::Color::White);
            quest_disp.set_text_size(22);
            quest_disp.set_text_color(fltk::enums::Color::Black);
            // endregion

            // region Setup the edit button & callback. Buttons not added to widget struct.
            let editbtn_x = quest_disp.x() + quest_disp.w() - 65;  // Button is sized 50 X 30
            let editbtn_y = quest_disp.y() + quest_disp.h() - 35;
            let mut editbtn = Button::new(editbtn_x, editbtn_y, 50, 30, "Edit");

            editbtn.set_callback(move |_| {
                println!("\n Edit button for Question #{} has been pressed. \n", qnum);
                qst_edit(qnum - 1);
            });
            // endregion

            // region Increment values and push/add items to WIDGETS struct
            box_y += QDISP_HEIGHT;    // Increment the question display widget position.
            qnum += 1;       // Increment the question display number.

            wdgts.qstn_boxes.push(quest_disp.clone());
            wdgts.scroll.add(&quest_disp);
            wdgts.scroll.add(&editbtn);
            // endregion
        }
        *WIDGETS.lock().unwrap() = wdgts.clone();    // Update the WIDGET global variable.
    }

    pub fn get_window_attrs(win: &Window) {
        // For debugging purposes.  Move to  lib.utils???

        let xxx = win.x();
        let yyy = win.y();
        let www = win.w();
        let hhh = win.h();

        println!("\n The position of the primary window is :  ({}, {}) \n", xxx, yyy);
        println!("The size of the primary window is :  ({}, {}) \n", www, hhh);
    }

    /// Check to see if a bank has been loaded or not.
    ///
    pub fn check_for_bank_loaded() -> bool {
        let mut usebank = Bank::new();
        {
            usebank = CURRENT_BANK.lock().unwrap().clone();
        }
        if usebank.bank_title.is_empty() {
            println!("\n No bank has been loaded. \n");  // Find a non-terminal way to display this.
            println!("   Please open a bank. \n");
            false
        } else {
            true
        }
    }

}


