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
            let mut previous: HashSet<&Cell> = HashSet::new();
            traverse(dictionary, &board, cell, &mut previous, cell.contents.to_string());
        }
    }

    println!("Final answers:");
    for word in FOUND_WORDS.lock().unwrap().iter() {
        println!("{}", word);
    }
}

fn traverse<'a>(dict: &Fst<Vec<u8>>, board: &'a Board, cell: &'a Cell, previous: &mut HashSet<&'a Cell>, value: String) {
    if NOT_WORDS.lock().unwrap().contains(&value) {
        return
    }

    match dictionary::prefix_search(dict, &value) {
        SearchResult::None => {
            println!("Not a word: {}", &value);
            NOT_WORDS.lock().unwrap().insert(value);
            return
        },
        SearchResult::Prefix => {
            println!("Valid prefix: {}", &value);
        },
        SearchResult::Word => {
            let mut set = FOUND_WORDS.lock().unwrap();

            if !set.contains(&value) {
                println!("Found word! {}", &value);
                set.insert(value.clone());
            }
        }
    };

    let neighbors = board.neighbors(cell, previous);

    for neighbor in neighbors {
        let mut value = value.clone();

        previous.insert(&cell);
        value.push(neighbor.contents);
        traverse(dict, board, neighbor, previous, value);
    }
}
