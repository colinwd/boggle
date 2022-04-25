mod board;
mod dictionary;

use std::collections::HashSet;

use board::Board;
use dictionary::SearchResult;
use fst::raw::Fst;

use crate::board::Cell;

fn main() {
    let dictionary = &dictionary::load();
    let board = Board::new();

    println!("{}", board);

    let first = board.first();

    let mut previous: HashSet<&Cell> = HashSet::new();
    previous.insert(first);

    traverse(dictionary, &board, first, &mut previous, first.contents.into());
}

fn traverse<'a>(dict: &Fst<Vec<u8>>, board: &'a Board, cell: &'a Cell, previous: &mut HashSet<&'a Cell>, value: String) -> Option<String> {
    let neighbors = board.neighbors(cell, previous);

    println!("I am cell {}, and my neighbors are {:?}", cell, neighbors);

    for neighbor in neighbors {
        let mut value = value.clone();
        let found = match dictionary::prefix_search(dict, value.as_ref()) {
            SearchResult::None => {
                println!("No match found for prefix {}", value);
                None
            },
            SearchResult::Prefix => {
                previous.insert(&cell);
                value.push(cell.contents);
                traverse(dict, board, neighbor, previous, value)
            },
            SearchResult::Word => Some(value)
        };

        if found.is_some() {
            println!("Found word! {}", found.unwrap())
        }
    }

    None
}
