use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert<I>(&mut self, iter: I) where I: Iterator<Item = char> {
        let mut curr = self;
        for c in iter {
            curr = curr.children.entry(c).or_default();
        }
    }

    fn dfs(&self, len: usize, sum: &mut usize) {
        let mut is_longest = true;
        for node in self.children.values() {
            is_longest = false;
            node.dfs(len + 1, sum);
        }
        if is_longest {
            *sum += len + 1;
        }
    }
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut tree = Trie::new();
        words.iter().for_each(|w| tree.insert(w.chars().rev()));
        let mut ans = 0;
        tree.dfs(0, &mut ans);
        ans as i32
    }
}
