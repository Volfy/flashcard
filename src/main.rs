// refer to https://github.com/schulke-214/connect-four for help
// flashcard program requirements:
// - Simple terminal gui
// * Vocabulary load Done! 
// - - Fix error handling and improve csvs 
// - app shows random word from dictionary
// - player has to input word (error check)
// - compare input with word match from dict
// - if wrong allow another input
// - if right show correct and show next card

// only use this for clean terminal
//#![allow(warnings)]
extern crate csv;
use csv::Reader;
use std::{error::Error,path::Path,io};

// holds possible language vocabs
enum LanguageChoice {
    EngGer,
    EngMlt,
    Fail, //Error handle instead ******
}

// finds the file depending on language choice
fn find_file(lang_choice: LanguageChoice) -> &'static Path {

    // match to enum and send reference of file
    match lang_choice {
        LanguageChoice::EngGer => Path::new("src/eng_ger.csv"), 
        LanguageChoice::EngMlt => Path::new("src/eng_mlt.csv"),
        // needs to be removed ******
        LanguageChoice::Fail => Path::new("src/eng_mlt.csv"), 
    }
}

// loads the csv file based on language choice
fn load_csv(lang_choice: LanguageChoice) -> Result<(), Box<dyn Error>> {
    // csv parser 
    let mut rdr = Reader::from_path(find_file(lang_choice))?;

    for record in rdr.records() {
        // test record
        let record = record?;
        // prints csv out (for testing)
        println!("Run: {} : {}\n",
        &record[0],
        &record[1]);
    }
    Ok(())
}

fn input_choice() -> u8 {
    let mut input = String::new();
    
    //read next line
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => println!("Error")
    }
    // splits off the new line \n
    input.split_off(input.len() -1);
    // parses and returns choice
    match input.parse() {
        Ok(val) => val,
        Err(_) => 0, }
}

fn main() {
    // informs player of choices
    println!("\nTo load English-German cards input 1.\nTo load English-Maltese cards input 2.\n");

    // defines language choice. todo: cleanup
    let lang_choice = match input_choice() { 
        1 => LanguageChoice::EngGer,
        2 => LanguageChoice::EngMlt,
        _ => LanguageChoice::Fail,
    };

    // loads up csv
    load_csv(lang_choice);
}
