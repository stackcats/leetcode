use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    min_len: usize,
    ndx: usize,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            min_len: usize::MAX,
            ndx: 0,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, s: String, i: usize) {
        let mut node = &mut self.root;
        if node.min_len > s.len() {
            node.min_len = s.len();
            node.ndx = i;
        }
        for c in s.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
            if node.min_len > s.len() {
                node.min_len = s.len();
                node.ndx = i;
            }
        }
    }

    fn find(&self, s: String) -> usize {
        let mut node = &self.root;
        for c in s.chars() {
            if let Some(t) = node.children.get(&c) {
                node = t;
            } else {
                break;
            }
        }
        node.ndx
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut t = Trie::new();
        words_container
            .iter()
            .enumerate()
            .for_each(|(i, w)| t.insert(w.chars().rev().collect(), i));
        words_query
            .iter()
            .map(|w| t.find(w.chars().rev().collect()) as i32)
            .collect()
    }
}
