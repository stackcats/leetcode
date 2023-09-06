impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowels: Vec<u8> = s.bytes().filter(is_vowel).collect();

        vowels.sort_unstable();

        let mut s = s.into_bytes();

        let mut vowels = vowels.into_iter();

        for c in &mut s {
            if is_vowel(c) {
                *c = vowels.next().unwrap();
            }
        }

        unsafe { String::from_utf8_unchecked(s) }
    }
}

fn is_vowel(&c: &u8) -> bool {
    match c {
        b'a' | b'e' | b'i' | b'o' | b'u' => true,
        b'A' | b'E' | b'I' | b'O' | b'U' => true,
        _ => false,
    }
}
