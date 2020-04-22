// refer to https://github.com/schulke-214/connect-four for help
// flashcard program requirements:
// - Simple terminal gui
// - - Improve CSVs (once other stuff is sorted out)
// - - allow loading custom csv
// - "youre close" message for typos
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
        format!("\nTo load {}-{} cards input {}.", self.lang_one, self.lang_two, self.selection)
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
fn load_language() -> Vec<(String, String)> {
    let eng_ger = Language::new(
        "English".to_string(), 
        "German".to_string(), 
        1, 
        load_csv(Path::new("voc/eng_ger.csv")));
    let eng_mlt = Language::new(
        "English".to_string(), 
        "Maltese".to_string(), 
        2, 
        load_csv(Path::new("voc/eng_mlt.csv")));
    
    println!("{}{}\n", 
        eng_ger.generate_menu_option(), 
        eng_mlt.generate_menu_option());

    // find a way to use the Language struct selection field as a pattern?
    loop{
        match user_input().parse().unwrap_or(0){ 
            1 => return eng_ger.dicti,
            2 => return eng_mlt.dicti,
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
        std::process::exit(0);
    }

    input
}

////////////////////////////////////////////////////////////

fn main() {
    
    println!("Welcome to my Flashcard program! (:q to quit at any time)");

    let mut loaded_dict = load_language();

    println!("How many words would you like to practice today?");
    
    let words_to_practice = user_input().parse().unwrap_or(0);

    println!("{:?}", loaded_dict);
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
        println!("What is the translation for: {} (:n to skip)", loaded_dict[x].0);

        loop {
            // load input function and compare
            // if correct go to next word, if wrong allow player to try again
            let input = user_input().to_lowercase();
            if  input == loaded_dict[x].1.to_lowercase() {
                println!("Good job!\n");
                break;
            } else if input != ":n" { 
                println!("Try again.");
                continue;
            } else {
                println!("Correct answer is {}.\n", loaded_dict[x].1);
                break;
            }
        }
    }
}