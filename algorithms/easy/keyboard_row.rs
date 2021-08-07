// https://leetcode.com/problems/keyboard-row/

use std::collections::HashMap;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut dt = HashMap::new();

        for c in "qwertyuiop".chars() {
            dt.insert(c, 1);
        }

        for c in "asdfghjkl".chars() {
            dt.insert(c, 2);
        }

        for c in "zxcvbnm".chars() {
            dt.insert(c, 3);
        }

        let mut ans = Vec::new();

        for word in words.iter() {
            let mut row = 0;
            for c in word.to_lowercase().chars() {
                if row == 0 {
                    row = dt[&c];
                } else if row != dt[&c] {
                    row = -1;
                }
            }

            if row != -1 {
                ans.push(word.clone());
            }
        }

        ans
    }
}
