use std::collections::HashSet;
use std::fmt::Display;

use std::{assert_matches, io};

const GAP: usize = 3;

/// Pole has GAP to adjust attempts left
pub struct Pole {
    the_answer: String,
    indices: HashSet<usize>,
    hist: Vec<char>,
    left: usize,
}

impl Pole {
    pub fn new(an_answer: String) -> Result<Self, PoleError> {
        let left = if an_answer.len() <= GAP {
            let msg = format!("Need more letters(>{GAP})");
            return Err(PoleError::Other(msg));
        } else {
            an_answer.len() + GAP
        };
        Ok(Self {
            the_answer: an_answer,
            indices: HashSet::new(),
            hist: vec![],
            left,
        })
    }

    pub fn try_next(&mut self, a_symbol: char) -> Result<(), PoleError> {
        if !(a_symbol.is_alphabetic() && a_symbol.is_ascii()) {
            return Err(PoleError::NotInRange);
        }
        if self.hist.contains(&a_symbol) {
            return Err(PoleError::Same);
        }

        let try_indicies: Vec<usize> = self
            .the_answer
            .chars()
            .map(|ch| ch.to_ascii_lowercase())
            .enumerate()
            .filter_map(|(i, ch)| if ch == a_symbol { Some(i) } else { None })
            .collect();

        self.indices.extend(try_indicies.clone());
        self.hist.push(a_symbol);

        if self.indices.len() == self.the_answer.len() {
            return Err(PoleError::Done(self.show()));
        }

        if try_indicies.is_empty() {
            self.left -= 1;
            if self.left == 0 {
                return Err(PoleError::Hanged);
            }
            return Err(PoleError::Missed(self.left));
        }

        Ok(())
    }

    pub fn show(&self) -> String {
        let out: Vec<_> = self
            .the_answer
            .chars()
            .enumerate()
            .map(|(i, ch)| if self.indices.contains(&i) { ch } else { '_' })
            .collect();
        let out: Vec<_> = out.iter().map(|c| c.to_string()).collect();
        out.join(" ")
    }
}

#[derive(Debug)]
pub enum PoleError {
    Missed(usize),
    Same,
    NotInRange,
    Hanged,
    Done(String),
    Other(String),
}

impl Display for PoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CatOrDog error")
    }
}

pub fn lower_single(symbols: &str) -> Result<char, io::Error> {
    let symbols = symbols.trim().to_ascii_lowercase();

    let mut chars = symbols.chars();

    match chars.next() {
        Some(ch) => {
            if chars.next() == None {
                Ok(ch)
            } else {
                Err(io::Error::other(String::from("Too many")))
            }
        }
        None => Err(io::Error::other(String::from("Not enough"))),
    }
}

#[test]
fn single() {
    let the_answer = String::from("hang");
    let mut pole = Pole::new(the_answer.clone()).unwrap();
    let next = the_answer.chars().next().unwrap();

    let tryed = pole.try_next(next).unwrap();

    assert!(pole.indices.len() == 1);
    assert_matches!(tryed, ());
}

#[test]
fn multiple() {
    let the_answer = String::from("hang");
    let mut pole = Pole::new(the_answer.clone()).unwrap();
    let mut the_answer = the_answer.chars();

    let next = the_answer.next().unwrap();
    pole.try_next(next).unwrap();
    let next = the_answer.next().unwrap();
    let tryed = pole.try_next(next).unwrap();

    assert!(pole.indices.len() == 2);
    assert!(tryed == ());
}

#[test]
fn multiple_but_guessed() {
    let the_answer = String::from("hang");
    let mut pole = Pole::new(the_answer.clone()).unwrap();
    let mut the_answer = the_answer.chars();

    let next = the_answer.next().unwrap();
    pole.try_next(next).unwrap();
    let next = the_answer.next().unwrap();
    pole.try_next(next).unwrap();
    let next = the_answer.next().unwrap();
    pole.try_next(next).unwrap();
    let next = the_answer.next().unwrap();
    let tryed = pole.try_next(next).unwrap_err();

    assert_matches!(tryed, PoleError::Done(_));
}

#[test]
fn multiple_but_hanged() {
    let the_answer = String::from("hang");
    let mut pole = Pole::new(the_answer.clone()).unwrap();
    let next = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];

    next.iter().for_each(|ch| {
        let tryed = pole.try_next(*ch);
        match ch {
            'i' => assert_matches!(tryed.unwrap_err(), PoleError::Missed(_)),
            'j' => assert_matches!(tryed.unwrap_err(), PoleError::Hanged),
            _ => (),
        }
    });
}

#[test]
fn guessed_on_last() {
    let the_answer = String::from("hang");
    let mut pole = Pole::new(the_answer.clone()).unwrap();
    let next = ['b', 'c', 'd', 'e', 'f', 'i', 'h', 'a', 'n', 'g'];

    next.iter().for_each(|ch| {
        let a = pole.try_next(*ch);
        match ch {
            'n' => assert_matches!(a.unwrap(), ()),
            'g' => {
                let a = a.unwrap_err();
                println!("{:?}", pole.hist);
                assert_matches!(a, PoleError::Done(_));
            }
            _ => (),
        }
    });
}

#[test]
fn attempts_left() {
    let the_answer = String::from("hang");
    let mut pole = Pole::new(the_answer.clone()).unwrap();
    let next = ['h', 'a', 'b'];

    next.iter().for_each(|ch| {
        pole.try_next(*ch).unwrap_or(());
    });

    assert!(pole.left == GAP + the_answer.len() - 1)
}

#[test]
fn pole_out() {
    let the_answer = String::from("Hang");
    let mut pole = Pole::new(the_answer.clone()).unwrap();
    let next = 'h';

    let a = pole.show();

    assert_eq!(a, String::from("_ _ _ _"));
    
    pole.try_next(next).unwrap();

    assert!(pole.show() == "H _ _ _");
}
