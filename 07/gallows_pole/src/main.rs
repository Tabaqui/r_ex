use std::io::{self, Write};

use gallows_pole::{Pole, PoleError, lower_single};

const THE_ANSWER: &str = "Hangman";

fn main() {
    let word_in = std::io::stdin();

    println!("Hi there");

    let pole = Pole::new(String::from(THE_ANSWER));

    match pole {
        Ok(mut pole) => loop {
            println!("{}\n", pole.show());
            let mut symbols = String::new();
            print!("Next guess is: ");
            io::stdout().flush().expect("Out got error");
            word_in.read_line(&mut symbols).expect("In got error");

            match lower_single(&symbols) {
                Ok(s) => match pole.try_next(s) {
                    Err(e) => match e {
                        PoleError::Missed(i) => println!("Try harder. Attempts left: {i}"),
                        PoleError::Same => println!("Tryed before."),
                        PoleError::NotInRange => println!("Try harder but use letter."),
                        PoleError::Hanged => {
                            println!("First time yeah?");
                            break;
                        }
                        PoleError::Done(word) => {
                            println!("> {word} <");
                            if word == String::from("H a n g m a n") {
                                println!("\nHangman! Hangman! Hold on a little while... I think my friends coming riding many miles...")
                            }
                            println!("\nCongrats! Tryed harder enough.\n");
                            println!("Please come again.");
                            break;
                        }
                        PoleError::Other(_) => println!("Someting got wrong. Anyway try harder."),
                    },
                    _ => (),
                },
                Err(e) => {
                    println!("{e}")
                }
            }
        },
        Err(e) => {
            if let PoleError::Other(msg) = e {
                println!("{:?}", msg);
            }
        }
    }
}
