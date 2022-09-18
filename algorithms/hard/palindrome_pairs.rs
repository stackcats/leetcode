use std::collections::HashMap;

fn is_palindrome(s: &[char]) -> bool {
    if s.len() == 0 {
        return true;
    }
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s[i] != s[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        for (i, word) in words.iter().enumerate() {
            map.insert(word, i);
        }

        let mut ans = Vec::new();
        for (i, word) in words.iter().enumerate() {
            let word: Vec<char> = word.chars().collect();
            for k in 0..=word.len() {
                let s1 = &word[0..k];
                let s2 = &word[k..];
                if is_palindrome(s1) {
                    if let Some(&j) = map.get(&s2.iter().rev().collect::<String>()) {
                        if i != j {
                            ans.push(vec![j as i32, i as i32]);
                        }
                    }
                }

                if !s2.is_empty() && is_palindrome(s2) {
                    if let Some(&j) = map.get(&s1.iter().rev().collect::<String>()) {
                        if i != j {
                            ans.push(vec![i as i32, j as i32]);
                        }
                    }
                }
            }
        }
        ans
    }
}
