fn word_to_int(word: &str) -> i32 {
    let word = word.as_bytes();
    let mut n = 0;
    for c in word {
        n = n * 10 + (c - b'a') as i32;
    }
    n
}

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let a = word_to_int(&first_word);
        let b = word_to_int(&second_word);
        let t = word_to_int(&target_word);
        a + b == t
    }
}
