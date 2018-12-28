use std::collections::HashMap;

struct TrieNode {
    keys: Box<HashMap<u8, Box<TrieNode>>>,
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

    fn add(&mut self, word: &[u8]) {
        let mut curr = &mut self.root;
        for &c in word.iter() {
            curr = moving(curr)
                .keys
                .entry(c)
                .or_insert(Box::new(TrieNode::new()));
        }
        curr.is_end = true;
    }

    fn search(&self, word: &[u8]) -> bool {
        let mut curr = &self.root;
        for c in word.iter() {
            if !curr.keys.contains_key(&c) {
                return false;
            }
            curr = curr.keys.get(&c).unwrap();
        }

        curr.is_end
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut d = vec![false; s.len() + 1];

        let mut trie = Trie::new();
        for word in word_dict.iter() {
            trie.add(word.as_bytes());
        }

        d[0] = true;

        let b = s.as_bytes();
        for i in 0..d.len() {
            for j in 0..i {
                let sub = &b[j..i];
                if d[j] && trie.search(sub) {
                    d[i] = true;
                    break;
                }
            }
        }

        d.pop().unwrap()
    }
}
