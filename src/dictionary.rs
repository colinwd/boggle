use fst::{Set, SetBuilder};
use fst::raw::Fst;
use std::fs::File;
use std::io::{self, BufRead};

pub fn load() -> Fst<Vec<u8>> {
    let dictionary = File::open("twl06-3-clean.txt").expect("Unable to read dictionary");
    let dictionary = io::BufReader::new(dictionary);

    let mut set = SetBuilder::memory();

    for line in dictionary.lines() {
        set.insert(line.unwrap().to_ascii_uppercase()).expect("Dictionary file must be sorted");
    }

    Set::new(set.into_inner().unwrap()).unwrap().into_fst()
}

pub fn prefix_search(fst: &Fst<Vec<u8>>, key: &str) -> SearchResult {
    let mut node = fst.root();

    for b in key.as_bytes() {
        match node.find_input(*b) {
            None => return SearchResult::None,
            Some(i) => {
                node = fst.node(node.transition_addr(i))
            }
        }
    }

    if node.is_final() {
        return SearchResult::Word
    } else {
        return SearchResult::Prefix
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum SearchResult {
    None,
    Prefix,
    Word
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_word() {
        let fst = load();
        assert_eq!(
            prefix_search(&fst, "WALK"),
            SearchResult::Word
        )
    }

    #[test]
    fn finds_prefix() {
        let fst = load();
        assert_eq!(
            prefix_search(&fst, "WA"),
            SearchResult::Prefix
        )
    }

    #[test]
    fn returns_none() {
        let fst = load();
        assert_eq!(
            prefix_search(&fst, "fndlsajiva"),
            SearchResult::None
        )
    }
}