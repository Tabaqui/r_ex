use std::io::{self, Write};

use gallows_pole_2::{Pole, Scaffold, lower_single};

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
                        Scaffold::Missed(i) => println!("Try harder. Attempts left: {i}"),
                        Scaffold::Same => println!("Tryed before."),
                        Scaffold::NotInRange => println!("Try harder but use letter."),
                        Scaffold::Hanged => {
                            println!("First time yeah?");
                            break;
                        }
                        Scaffold::Done(word) => {
                            println!("> {word} <");
                            if word == String::from("H a n g m a n") {
                                println!(
                                    "\nHangman! Hangman! Hold it a little while... I think  I see my friends coming riding many a mile..."
                                )
                            }
                            println!("\nCongrats! Tryed harder enough.\n");
                            println!("Please come again.");
                            break;
                        }
                    },
                    _ => (),
                },
                Err(e) => {
                    println!("{e}")
                }
            }
        },
        Err(_) => {
            println!("U'r definitely not the fist time");
        }
    }
}
