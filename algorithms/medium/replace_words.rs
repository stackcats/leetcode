use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode{
            children: HashMap::new(),
            is_end: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, s: &str) {
        let mut curr = &mut self.root;
        for c in s.chars() {
            curr = curr.children.entry(c).or_insert(TrieNode::new());
        }
        curr.is_end = true;
    }

    fn search(&self, s: &str) -> Option<String> {
        let mut curr = &self.root;
        let mut tmp = String::new();
        for c in s.chars() {
            if let Some(v) = curr.children.get(&c) {
                tmp.push(c);
                if v.is_end {
                    return Some(tmp);
                }
                curr = v;
            } else {
                return None;
            }
        }

        None
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut t = Trie::new();
        for w in &dictionary {
            t.insert(w);
        }

        let arr: Vec<&str> = sentence.split(' ').collect();
        let mut ans = Vec::new();
        for s in arr {
            ans.push(t.search(s).unwrap_or(s.to_string()))
        }

        ans.join(" ")
    }
}
