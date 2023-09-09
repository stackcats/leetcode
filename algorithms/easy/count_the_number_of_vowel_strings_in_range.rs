fn is_vowel(u: u8) -> bool {
    match u {
        b'a' | b'e' | b'i' | b'o' | b'u' => true,
        _ => false,
    }
}
impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        words[(left as usize)..=(right as usize)]
            .into_iter()
            .filter(|w| {
                let bs = w.as_bytes();
                is_vowel(bs[0]) && is_vowel(bs[bs.len() - 1])
            })
            .count() as i32
    }
}
