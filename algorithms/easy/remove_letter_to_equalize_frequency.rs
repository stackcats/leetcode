use std::collections::HashSet;

fn is_equal(arr: &[u8]) -> bool {
    let set: HashSet<_> = arr.iter().filter(|v| **v != 0).collect();
    set.len() == 1
}

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut map = vec![0; 26];
        for b in word.as_bytes() {
            map[(b - b'a') as usize] += 1;
        }

        for i in 0..map.len() {
            if map[i] > 0 {
                map[i] -= 1;
                if is_equal(&map) {
                    return true;
                }
                map[i] += 1;
            }
        }

        false
    }
}
