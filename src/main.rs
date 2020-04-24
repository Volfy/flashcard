// uses some code/ideas from https://github.com/schulke-214/connect-four

// Basic flashcard program written to practice Rust, 
// TODO: (* done - not done)
// - "youre close" message for typos
// - accept alternative spellings eg umlauts
// - fix the panic on custom loading
// - improve performance by loading dictionaries only when needed
// * Simple terminal gui
// * * Improve CSVs (once other stuff is sorted out)
// * * allow loading custom csv
// * Vocabulary load Done! 
// * * Figure out how to autogenerate menu options? is this necessary?
// * app shows random word from dictionary
// * * avoid repetitions
// * player has to input word (error check)
// * * input functions need to be combined
// * * compare input with word match from dict
// * * if wrong allow another input or allow player to go to next card
// * * if right show correct and show next card
// * allow player to quit

use csv::{self, Reader};
use std::{path::Path,io};
use rand::{thread_rng, seq::SliceRandom};

// defines a language to make loading info easier
struct Language {
    lang_one: String,
    lang_two: String,
    selection: i8,
    dicti: Vec<(String, String)>,
}
impl Language {
    fn new(lang_one: String, lang_two: String, selection: i8, dicti: Vec<(String, String)>) -> Language {
        Language {
            lang_one,
            lang_two,
            selection,
            dicti,
        }
    }
    fn generate_menu_option(&self) -> String {
        format!("\nTo load {} to {} cards input {}.", self.lang_one, self.lang_two, self.selection)
    }
}

fn clear_screen(is_b4: bool) {
    if is_b4 {
        println!("**************************************************************************\n");
    } else {
        println!("\x1B[2J**************************************************************************\n");
    }
    
}

// loads the csv file based on language choice
fn load_csv(p: &Path) -> Vec<(String, String)> {

    // initialize vector
    let mut loaded_dict: Vec<(String, String)> = vec![];

    // csv parser 
    let mut rdr = Reader::from_path(p).unwrap();

    for record in rdr.records() {
        // disregards errors
        let record = record.unwrap();
        
        // places everything into a vector
        loaded_dict.push((record[0].to_string(), record[1].to_string()));
    }

    loaded_dict
}

// defines languages, asks player for choice, returns the loaded dictionary
fn load_language() -> (Vec<(String, String)>, String) {
    let ita_eng = Language::new(
        "Italian".to_string(), 
        "English".to_string(), 
        1, 
        load_csv(Path::new("voc/ita_eng.csv")));
    let ger_eng = Language::new(
        "German".to_string(), 
        "English".to_string(), 
        2, 
        load_csv(Path::new("voc/ger_eng.csv")));
    let fra_eng = Language::new(
        "French".to_string(), 
        "English".to_string(), 
        3, 
        load_csv(Path::new("voc/fra_eng.csv")));
    let nlx_eng = Language::new(
        "Dutch".to_string(), 
        "English".to_string(), 
        4, 
        load_csv(Path::new("voc/nlx_eng.csv")));
    let rus_eng = Language::new(
        "Russian".to_string(), 
        "English".to_string(), 
        5, 
        load_csv(Path::new("voc/rus_eng.csv")));
    
    println!("{}{}{}{}{}\nEnter 0 to use custom csv.\n", 
        ita_eng.generate_menu_option(), 
        ger_eng.generate_menu_option(),
        fra_eng.generate_menu_option(),
        nlx_eng.generate_menu_option(),
        rus_eng.generate_menu_option());

    // find a way to use the Language struct selection field as a pattern?
    loop{
        match user_input().parse().unwrap_or(-1){ 
            1 => return (ita_eng.dicti, ita_eng.lang_one),
            2 => return (ger_eng.dicti, ger_eng.lang_one),
            3 => return (fra_eng.dicti, fra_eng.lang_one),
            4 => return (nlx_eng.dicti, nlx_eng.lang_one),
            5 => return (rus_eng.dicti, rus_eng.lang_one),
            0 => return ({
                // code panics if bad input. see if there's any way to make this more friendly.
                println!("Please enter the filepath for your csv (eg. voc/<name>.csv).");
                load_csv(&Path::new(&user_input()))
            }, "Custom".to_string()),
            _ => {
                println!("Invalid choice.");
                continue;
            }
        };
    };
}

// loads input for choice/word
fn user_input() -> String {
    let mut input = String::new();
    // take user input
    io::stdin().read_line(&mut input).unwrap();
    // clean newline \n input
    input.split_off(input.len() -1);

    if input == ":q" {
        println!("Thank you, bye!");
        std::process::exit(0);
    }

    input
}

////////////////////////////////////////////////////////////

fn main() {
    clear_screen(true);
    println!("Welcome to my Flashcard program! (:q to quit at any time)");

    let loaded_dict = load_language();

    println!("How many words would you like to practice today?");
    
    let words_to_practice = user_input().parse().unwrap_or(0);

    clear_screen(false);
    println!("Practicing {} words from {} today.\n", words_to_practice, loaded_dict.1);

    // get rid of names
    let mut loaded_dict = loaded_dict.0;
    // rng
    let mut rng = thread_rng();
    // shuffles dictionary randomly
    loaded_dict.shuffle(&mut rng);
    
    
    // loop main function until necessary
    for i in 0..words_to_practice {
        
        // allows wrap around
        let x = i % loaded_dict.len();
        // keeps things random
        if i % loaded_dict.len() == 0 {loaded_dict.shuffle(&mut rng);}
        clear_screen(true);
        println!("What is the translation for: \"{}\" (:n to skip)", loaded_dict[x].0);

        loop {
            // load input function and compare
            // if correct go to next word, if wrong allow player to try again
            let input = user_input().to_lowercase();
            if  input == loaded_dict[x].1.to_lowercase() {
                clear_screen(false);
                println!("Good job! \"{}\" is \"{}\".\n", loaded_dict[x].0, loaded_dict[x].1);
                break;
            } else if input != ":n" { 
                println!("Try again.");
                continue;
            } else {
                clear_screen(false);
                println!("The correct answer for \"{}\" is \"{}\".\n", loaded_dict[x].0, loaded_dict[x].1);
                break;
            }
        }
    }
    clear_screen(true);
    println!("Thank you, bye!");
}