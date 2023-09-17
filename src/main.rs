mod board;
mod dictionary;

#[macro_use]
extern crate lazy_static;

use std::{collections::HashSet, sync::Mutex};

use board::Board;
use dictionary::SearchResult;
use fst::raw::Fst;
use std::io::Write;

use crate::board::Cell;

lazy_static! {
    static ref FOUND_WORDS: Mutex<HashSet<String>> = {
        Mutex::new(HashSet::new())
    };
}

fn main() {
    let dictionary = &dictionary::load();
    let board = Board::new();

    for row in &board.slots {
        for cell in row {
            traverse(dictionary, &board, cell, vec!(cell));
        }
    }

    println!("{}", board);

    println!("Final answers:");
    let mut answers: Vec<String> = FOUND_WORDS.lock().unwrap().clone().into_iter().collect();
    answers.sort();

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    for word in answers {
        writeln!(lock, "{}", word).unwrap();
    }
}

/// Recursively traverse through the board. Tracks previously visited cells to avoid reusing, as is the way of Boggle.
fn traverse<'a>(dict: &Fst<Vec<u8>>, board: &'a Board, current_cell: &'a Cell, current_path: Vec<&'a Cell>) {
    let possible_word = current_path
        .iter()
        .map(|c| c.contents)
        .collect::<String>();

    match dictionary::prefix_search(dict, &possible_word) {
        // end early, as our current path does not form either a partial or whole word
        SearchResult::None => return,

        // we could find something...
        SearchResult::Prefix => {},
        
        // we found something!
        SearchResult::Word => {
            let mut set = FOUND_WORDS.lock().unwrap();

            if !set.contains(&possible_word) {
                set.insert(possible_word.clone());
            }
        }
    };

    let neighbors = board.neighbors(current_cell, &current_path);

    for neighbor in neighbors {
        let mut current_path = current_path.clone();

        current_path.push(neighbor);
        traverse(dict, board, neighbor, current_path);
    }
}
