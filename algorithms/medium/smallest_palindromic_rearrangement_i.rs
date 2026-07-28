impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut mp = vec![0; 26];
        for i in 0..s.len() / 2 {
            mp[(s[i] - b'a') as usize] += 1;
        }

        let mut left = String::new();
        for i in 0..26 {
            for _ in 0..mp[i] {
                left.push((i as u8 + b'a') as char);
            }
        }

        let right: String = left.chars().rev().collect();
        if s.len() % 2 == 1 {
            left.push(s[s.len() / 2] as char);
        }

        left + &right
    }
}
