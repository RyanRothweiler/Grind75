#![allow(dead_code, unused_variables, unused_imports)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    valid_word: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            valid_word: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut root = self;

        for c in word.chars() {
            let entry = root.children.entry(c).or_insert(Trie::new());
            root = entry;
        }

        root.valid_word = true;
    }

    fn search(&mut self, word: String) -> bool {
        let mut root = self;

        for c in word.chars() {
            match root.children.get_mut(&c) {
                Some(r) => {
                    root = r;
                }
                None => {
                    return false;
                }
            };
        }

        return root.valid_word;
    }

    fn starts_with(&mut self, prefix: String) -> bool {
        let mut root = self;

        for c in prefix.chars() {
            match root.children.get_mut(&c) {
                Some(r) => {
                    root = r;
                }
                None => {
                    return false;
                }
            };
        }

        return true;
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let mut t = Trie::new();
        t.insert("hey".into());

        assert_eq!(t.search("hey".into()), true);
        assert_eq!(t.search("heyyy".into()), false);
        assert_eq!(t.search("h".into()), false);
    }
}
