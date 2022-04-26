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
            previous.insert(cell);
            traverse(dictionary, &board, cell, &mut previous, String::new());
        }
    }

    println!("Final answers:");
    for word in FOUND_WORDS.lock().unwrap().iter() {
        println!("{}", word);
    }
}

fn traverse<'a>(dict: &Fst<Vec<u8>>, board: &'a Board, cell: &'a Cell, previous: &mut HashSet<&'a Cell>, value: String) -> Option<String> {
    if NOT_WORDS.lock().unwrap().contains(&value) {
        return None
    }

    //println!("Traversing! Cell {}, previous {:?}, value {}", cell, previous, value);

    let neighbors = board.neighbors(cell, previous);

    for neighbor in neighbors {
        let mut value = value.clone();

        let found = match dictionary::prefix_search(dict, value.as_ref()) {
            SearchResult::None => {
                NOT_WORDS.lock().unwrap().insert(value);
                None
            },
            SearchResult::Prefix => {
                previous.insert(&cell);
                value.push(cell.contents);
                traverse(dict, board, neighbor, previous, value);
                None
            },
            SearchResult::Word => Some(value)
        };

        if found.is_some() {
            let found = found.unwrap();
            let mut set = FOUND_WORDS.lock().unwrap();

            if !set.contains(&found) {
                set.insert(found);
            }
        }
    }

    None
}
