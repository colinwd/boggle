use fst::Set;
use fst::raw::Fst;

pub fn load() -> Fst<Vec<u8>> {
    Set::new(std::fs::read("dict.fst").unwrap()).unwrap().into_fst()
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