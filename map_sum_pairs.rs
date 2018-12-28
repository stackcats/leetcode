use std::collections::HashMap;

struct TrieNode {
    keys: Box<HashMap<u8, TrieNode>>,
    sum: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            keys: Box::new(HashMap::new()),
            sum: 0,
        }
    }
}

struct MapSum {
    root: TrieNode,
}

fn rec(trie: &TrieNode) -> i32 {
    trie.keys.values().fold(trie.sum, |acc, x| acc + rec(x))
}

impl MapSum {
    fn new() -> Self {
        MapSum {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        let mut curr = &mut self.root;
        for c in key.as_bytes() {
            let tmp = curr;
            curr = tmp.keys.entry(*c).or_insert(TrieNode::new());
        }

        curr.sum = val;
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut curr = &self.root;
        for c in prefix.as_bytes() {
            if !curr.keys.contains_key(&c) {
                return 0;
            }
            curr = curr.keys.get(&c).unwrap();
        }

        rec(curr)
    }
}
