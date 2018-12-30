// https://leetcode.com/problems/first-unique-character-in-a-string/

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut arr = vec![0; 26];
        for b in s.as_bytes() {
            let i = b - b'a';
            arr[i as usize] += 1;
        }

        for (ndx, b) in s.as_bytes().iter().enumerate() {
            let i = b - b'a';
            if arr[i as usize] == 1 {
                return ndx as i32;
            }
        }

        -1
    }
}
