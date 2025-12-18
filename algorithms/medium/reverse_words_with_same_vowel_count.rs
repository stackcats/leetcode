fn count_vowels(s: &[u8]) -> i32 {
    s.iter().fold(0, |acc, c| match c {
        b'a' | b'e' | b'i' | b'o' | b'u' => acc + 1,
        _ => acc,
    })
}

impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        let mut it = unsafe { s.as_bytes_mut().split_mut(|c| *c == b' ') };
        let ct = count_vowels(it.next().unwrap());
        for t in it {
            if count_vowels(t) == ct {
                t.reverse();
            }
        }
        s
    }
}
