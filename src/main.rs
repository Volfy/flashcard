// refer to https://github.com/schulke-214/connect-four for help
// flashcard program requirements:
// - Simple terminal gui
// * Vocabulary load Done! 
// - - Improve CSVs
// - - Figure out how to autogenerate menu options
// - app shows random word from dictionary
// - player has to input word (error check)
// - compare input with word match from dict
// - if wrong allow another input
// - if right show correct and show next card
extern crate csv;
use csv::Reader;
use std::{error::Error,path::Path,io};

// holds possible language vocabs
enum LanguageChoice {
    EngGer,
    EngMlt,
}
impl LanguageChoice {
    // finds the file depending on language choice
    fn find_file(&self) -> &'static Path {

        // match to enum and send path to file
        match &self {
            LanguageChoice::EngGer => Path::new("voc/eng_ger.csv"), 
            LanguageChoice::EngMlt => Path::new("voc/eng_mlt.csv"),
        }
    }

    // loads the csv file based on language choice
    fn load_csv(&self) -> Result<(), Box<dyn Error>> {
        // csv parser 
        let mut rdr = Reader::from_path(&self.find_file())?;

        for record in rdr.records() {
            // test record in case of error
            let record = record?;
            // prints csv out (for testing)
            println!("{} : {}\n",
            &record[0],
            &record[1]);
        }
        Ok(())
    }
}

///////////////////////////////////////////////////////////
// processes input from player for choosing language
fn input_choice() -> u8 {
    let mut input = String::new();
    let return_val: u8;

    loop{
        // in case a number isn't entered, this is cleaned
        input.clear();

        //read next line
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Error");
                std::process::exit(0);
            }
        }
        // splits off the new line \n
        input.split_off(input.len() -1);

        // parses and returns choice
        return_val = match input.parse() {
            Ok(val) => val,
            // if not a number, print error message
            Err(_) => {
                println!("Please enter a number.");
                continue;
            } 
        };
        // if ok stop loop and return return_val, if not keep asking
        break;
    };
    return return_val;
}

////////////////////////////////////////////////////////////

fn main() {
    // informs player of available choices
    println!("\nTo load English-German cards input 1.");
    println!("To load English-Maltese cards input 2.\n");

    let lang_choice: LanguageChoice;
    // in loop until it reaches correct input, then loads csv
    loop {
        // player makes language choice. 
        lang_choice = match input_choice() { 
            1 => LanguageChoice::EngGer,
            2 => LanguageChoice::EngMlt,
            _ => {
                println!("Invalid choice.");
                continue;
            }
        };
        break;
    };
    // load the csv
    lang_choice.load_csv().expect("Error");
}
