use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    fn insert(&mut self, s: String) {
        let mut node = &mut self.root;
        for c in s.chars() {
            node = node.children.entry(c).or_default();
        }
    }

    fn longest_prefix(&self, s: String) -> i32 {
        let mut node = &self.root;
        let mut ct = 0;
        for c in s.chars() {
            if let Some(t) = node.children.get(&c) {
                node = t;
                ct += 1;
            } else {
                break;
            }
        }
        ct
    }
}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut t = Trie::new();
        arr1.iter().for_each(|n| t.insert(n.to_string()));
        arr2.iter()
            .fold(0, |acc, n| t.longest_prefix(n.to_string()).max(acc))
    }
}
