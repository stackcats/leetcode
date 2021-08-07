fn is_vowel(c: char) -> bool {
    let c = c.to_ascii_lowercase();
    c == 'a' || c == 'e' || c == 'i' || c == 'u' || c == 'o'
}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s == "" {
            return "".to_string();
        }
        let mut arr: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = arr.len() - 1;
        while i < j {
            if !is_vowel(arr[i]) {
                i += 1;
                continue;
            }
            if !is_vowel(arr[j]) {
                j -= 1;
                continue;
            }
            if i < j {
                let t = arr[i];
                arr[i] = arr[j];
                arr[j] = t;
            }
            i += 1;
            j -= 1;
        }
        arr.iter().collect()
    }
}
