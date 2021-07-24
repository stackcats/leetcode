fn is_differ_by_one(s: &[u8], t: &[u8]) -> bool {
    let mut is_diff = false;
    for i in 0..s.len() {
        if s[i] != t[i] {
            if !is_diff {
                is_diff = true;
            } else {
                return false;
            }
        }
    }
    is_diff
}

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut words = Vec::new();
        for l in 1..=s.len() {
            for i in 0..=(s.len() - l) {
                words.push(&s[i..(i + l)]);
            }
        }

        let mut ans = 0;
        for word in words.iter() {
            let l = word.len();
            for i in 0..=(t.len() - l) {
                if is_differ_by_one(word, &t[i..(i + l)]) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
