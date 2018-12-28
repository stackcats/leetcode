// https://leetcode.com/problems/longest-word-in-dictionary/

use std::collections::HashMap;

struct TrieNode {
    keys: Box<HashMap<char, TrieNode>>,
    word: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            keys: Box::new(HashMap::new()),
            word: None,
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

    fn add(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.chars() {
            curr = curr.keys.entry(c).or_insert(TrieNode::new());
        }
        curr.word = Some(word);
    }
}

fn dfs(trie: &TrieNode, ct: usize, ans: &mut String) {
    if ans.len() < ct {
        *ans = trie.word.unwrap().clone();
    } else if ans.len() == ct {
        if *ans > trie.word.unwrap() {
            *ans = trie.word.unwrap().clone();
        }
    }

    for child in trie.keys.values() {
        if child.word.is_some() {
            dfs(&child, ct + 1, ans);
        }
    }
}

impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        let mut trie = Trie::new();

        let mut ans = "".to_string();

        for word in words.iter() {
            trie.add(word.to_string());
        }

        for child in trie.root.keys.values() {
            if child.word.is_some() {
                dfs(&child, 1, &mut ans);
            }
        }

        ans
    }
}
