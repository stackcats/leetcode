fn aux(s: &str, t: &str) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        if s[i] == t[j] {
            j += 1;
        }
        i += 1;
    }

    j == t.len()
}

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut ans = String::new();
        for t in dictionary {
            if aux(&s, &t) {
                if ans.len() < t.len() || ans.len() == t.len() && ans > t {
                    ans = t;
                }
            }
        }
        ans
    }
}
