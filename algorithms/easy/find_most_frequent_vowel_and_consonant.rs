impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut max_vowel = 0;
        let mut max_conso = 0;
        let mut mp = vec![0; 26];
        for c in s.chars() {
            let n = (c as u8 - b'a') as usize;
            mp[n] += 1;
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => max_vowel = max_vowel.max(mp[n]),
                _ => max_conso = max_conso.max(mp[n]),
            }
        }
        max_vowel + max_conso
    }
}
