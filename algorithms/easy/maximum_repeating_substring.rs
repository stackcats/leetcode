impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut ans = 0;
        let mut longest = 0;
        for i in 0..sequence.len() {
            let mut is_equal = false;
            for j in (i..sequence.len()).step_by(word.len()) {
                if j + word.len() > sequence.len() {
                    break;
                }
                if &sequence[j..j + word.len()] == word {
                    if is_equal == true {
                        longest += 1;
                    } else {
                        is_equal = true;
                        longest = 1;
                    }
                } else {
                    break;
                }
            }
            ans = ans.max(longest);
        }
        ans
    }
}
