// refer to https://github.com/schulke-214/connect-four for help
// flashcard program requirements:
// - Simple terminal gui
// * Vocabulary load Done! 
// - - Improve CSVs (once other stuff is sorted out)
// - - Figure out how to autogenerate menu options? is this necessary?
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

            // should be placed into a vector or something
            println!("{} : {}\n",
            &record[0],
            &record[1]);
        }
        Ok(())
    }
}

///////////////////////////////////////////////////////////
// processes input from player for choosing language
fn input_choice() -> LanguageChoice {
    let mut input = String::new();

    // informs player of available choices
    println!("\nTo load English-German cards input 1.");
    println!("To load English-Maltese cards input 2.\n");

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

        // match with enum to ensure validity and then return LanguageChoice type
        match {
            //parses input and returns if number
            match input.parse() {
                Ok(val) => val,
                // if not a number, print error message
                Err(_) => {
                    println!("Please enter a number.");
                    continue;
                } 
            } 
        }   { 
            // if number was provided check that it's a valid selection (we are matching the result of the match)
            1 => return LanguageChoice::EngGer,
            2 => return LanguageChoice::EngMlt,
            _ => {
                println!("Invalid choice.");
                continue;
            }
        };
    };
}

////////////////////////////////////////////////////////////

fn main() {
    // displays choices, loads the csv and checks for error
    input_choice().load_csv().expect("Error");
}
