use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    index: i32,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            index: -1,
        }
    }
}


struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self { root: TrieNode::new() }
    }

    fn insert(&mut self, s: String, index: i32) {
        let mut curr = &mut self.root;
        for c in s.chars() {
            curr = curr.children.entry(c).or_insert(TrieNode::new());
            curr.index = curr.index.max(index);
        }
    }


    fn search(&self, s: String) -> i32 {
        let mut curr = &self.root;
        for c in s.chars() {
            if let Some(t) = curr.children.get(&c) {
                curr = t;
            } else {
                return -1;
            }
        }

        curr.index
    }
}

struct WordFilter {
    tree: Trie
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {

    fn new(words: Vec<String>) -> Self {
        let mut tree = Trie::new();
        for (i, w) in words.into_iter().enumerate() {
            for j in 0..w.len() {
                tree.insert(format!("{}.{}", &w[j..], w), i as i32);
            }
        }
        Self { tree }
    }
    
    fn f(&self, prefix: String, suffix: String) -> i32 {
        self.tree.search(format!("{}.{}", suffix, prefix))
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */
