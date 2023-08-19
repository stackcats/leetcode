impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut revs = vec![];
        for w in &words {
            revs.push(w.chars().rev().collect::<String>());
        }
        let mut ct = 0;
        for i in 0..words.len() - 1 {
            for j in i + 1..words.len() {
                if words[i] == revs[j] {
                    ct += 1;
                }
            }
        }
        ct
    }
}
