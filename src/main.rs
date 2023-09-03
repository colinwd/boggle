mod board;
mod dictionary;

#[macro_use]
extern crate lazy_static;

use std::{collections::HashSet, sync::Mutex};

use board::Board;
use dictionary::SearchResult;
use fst::raw::Fst;

use crate::board::Cell;

lazy_static! {
    static ref FOUND_WORDS: Mutex<HashSet<String>> = {
        Mutex::new(HashSet::new())
    };

    static ref NOT_WORDS: Mutex<HashSet<String>> = {
        Mutex::new(HashSet::new())
    };
}

fn main() {
    let dictionary = &dictionary::load();
    let board = Board::new();

    println!("{}", board);

    for row in &board.slots {
        for cell in row {
            traverse(dictionary, &board, cell, vec!(cell));
        }
    }

    println!("{}", board);

    println!("Final answers:");
    let mut answers: Vec<String> = FOUND_WORDS.lock().unwrap().clone().into_iter().collect();
    answers.sort();

    for word in answers {
        println!("{}", word);
    }
}

/// Recursively traverse through the board. Tracks previously visited cells to avoid reusing, as is the way of Boggle.
fn traverse<'a>(dict: &Fst<Vec<u8>>, board: &'a Board, current_cell: &'a Cell, current_path: Vec<&'a Cell>) {
    let possible_word = current_path
        .iter()
        .map(|c| c.contents)
        .collect::<String>();

    if NOT_WORDS.lock().unwrap().contains(&possible_word) {
        return
    }

    match dictionary::prefix_search(dict, &possible_word) {
        SearchResult::None => {
            println!("Not a word {}", possible_word);
            NOT_WORDS.lock().unwrap().insert(possible_word);
            return
        },
        SearchResult::Prefix => {},
        SearchResult::Word => {
            let mut set = FOUND_WORDS.lock().unwrap();

            if !set.contains(&possible_word) {
                println!("Found word! {}", &possible_word);
                set.insert(possible_word.clone());
            }
        }
    };

    let neighbors = board.neighbors(current_cell, &current_path);

    println!("Cell {} in possible word {} has neighbors {:?}", current_cell, possible_word, neighbors);

    // TODO: `previous` is currently not working as intended, seemingly holding onto old values,
    // even when we return early and move back up the stack.
    // Perhaps we can change the `previous` and `possible_word` implementation and unify them into an ordered set of some kind.

    for neighbor in neighbors {
        let mut current_path = current_path.clone();

        current_path.push(neighbor);
        traverse(dict, board, neighbor, current_path);
    }
}
