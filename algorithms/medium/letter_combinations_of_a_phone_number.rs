use std::collections::HashMap;

fn dfs(
    digits: &[char],
    m: &HashMap<char, Vec<char>>,
    ndx: usize,
    curr: &mut String,
    ans: &mut Vec<String>,
) {
    if ndx >= digits.len() {
        if curr.len() == digits.len() {
            ans.push(curr.clone());
        }

        return;
    }

    for i in ndx..digits.len() {
        let d = digits[i];
        for c in &m[&d] {
            let mut s = curr.clone();
            s.push(*c);
            dfs(digits, m, i + 1, &mut s, ans);
        }
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits == "" {
            return vec![];
        }
        let digits: Vec<char> = digits.chars().collect();
        let mut m = HashMap::new();
        m.insert('2', vec!['a', 'b', 'c']);
        m.insert('3', vec!['d', 'e', 'f']);
        m.insert('4', vec!['g', 'h', 'i']);
        m.insert('5', vec!['j', 'k', 'l']);
        m.insert('6', vec!['m', 'n', 'o']);
        m.insert('7', vec!['p', 'q', 'r', 's']);
        m.insert('8', vec!['t', 'u', 'v']);
        m.insert('9', vec!['w', 'x', 'y', 'z']);
        let mut ans = Vec::new();
        let mut s = String::new();
        dfs(&digits, &m, 0, &mut s, &mut ans);
        ans
    }
}
