fn is_vowel(c: u8) -> bool {
    c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u'
}

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut ct = vec![0; 26];
        let mut ndx = vec![s.len(); 26];
        let mut arr = Vec::new();
        let s = s.as_bytes();
        for i in 0..s.len() {
            if !is_vowel(s[i]) {
                continue;
            }
            let j = (s[i] - b'a') as usize;
            ct[j] += 1;
            ndx[j] = ndx[j].min(i);
            arr.push(s[i]);
        }

        arr.sort_by(|a, b| {
            let i = (a - b'a') as usize;
            let j = (b - b'a') as usize;
            if ct[i] == ct[j] {
                ndx[i].cmp(&ndx[j])
            } else {
                ct[j].cmp(&ct[i])
            }
        });

        let mut j = 0;
        let mut ans = String::new();
        for i in 0..s.len() {
            if is_vowel(s[i]) {
                ans.push(arr[j] as char);
                j += 1;
            } else {
                ans.push(s[i] as char);
            }
        }

        ans
    }
}
