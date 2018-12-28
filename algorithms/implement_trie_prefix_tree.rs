// https://leetcode.com/problems/implement-trie-prefix-tree

use std::collections::HashMap;

struct TrieNode {
    keys: Box<HashMap<char, Box<TrieNode>>>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            keys: Box::new(HashMap::new()),
            is_end: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

fn moving<T>(t: T) -> T {
    t
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = moving(node)
                .keys
                .entry(c)
                .or_insert(Box::new(TrieNode::new()));
        }

        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            if !node.keys.contains_key(&c) {
                return false;
            }
            node = node.keys.get(&c).unwrap();
        }

        node.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            if !node.keys.contains_key(&c) {
                return false;
            }
            node = node.keys.get(&c).unwrap();
        }

        true
    }
}
