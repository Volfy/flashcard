// refer to https://github.com/schulke-214/connect-four for help
// flashcard program requirements:
// - Simple terminal gui
// * Vocabulary load Done! 
// - - Improve CSVs (once other stuff is sorted out)
// - - allow loading custom csv
// - - Figure out how to autogenerate menu options? is this necessary?
// * app shows random word from dictionary
// - - avoid repetitions
// * player has to input word (error check)
// * * input functions need to be combined
// * * compare input with word match from dict
// * - if wrong allow another input or allow player to go to next card
// * * if right show correct and show next card
// - allow player to quit
// - "youre close" message for typos

use csv::{self, Reader};
use std::{path::Path,io};
use rand::{thread_rng, Rng};

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

fn load_languages() -> Vec<(String, String)> {
    let eng_ger = Language::new(
        "English".to_string(), 
        "German".to_string(), 
        1, 
        load_csv(Path::new("voc/eng_ger.csv")));
    let eng_ger = Language::new(
        "English".to_string(), 
        "Maltese".to_string(), 
        2, 
        load_csv(Path::new("voc/eng_mlt.csv")));

    println!("\nTo load {}-{} cards input {}.", eng_ger.lang_one, eng_ger.lang_two, eng_ger.selection);
    println!("To load {}-{} cards input {}.\n", eng_mlt.lang_one, eng_mlt.lang_two, eng_mlt.selection);

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


fn user_input() -> String {
    let mut input = String::new();
    // take user input
    io::stdin().read_line(&mut input).unwrap();
    // clean newline \n input
    input.split_off(input.len() -1);

    input
}

////////////////////////////////////////////////////////////

fn main() {
    // to keep clutter out of main, but could be done in main
    let loaded_dict = load_languages();

    println!("{:?}", loaded_dict);
    // rng
    let mut rng = thread_rng();
    
    // loop main function until necessary
    for _i in 0..10 {
        // random number in range of dict length
        let rdm = rng.gen_range(0, loaded_dict.len());
        println!("What is the translation for: {}", loaded_dict[rdm].0);

        loop {
            // load input function and compare
            // if correct go to next word, if wrong allow player to try again
            if user_input().to_lowercase() == loaded_dict[rdm].1.to_lowercase() {
                println!("Good job!");
                break;
            } else { 
                println!("Try again.");
                continue;
            }
        }
    }
}
