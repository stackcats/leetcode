// https://leetcode.com/submissions/detail/195385930/

impl Solution {
    pub fn next_greatest_letter(letters: &mut Vec<char>, target: &mut char) -> char {
        let mut i = 0;
        let mut j = letters.len() as i32 - 1;
        while i <= j {
            let mid = i + (j - i) / 2;
            if letters[mid as usize] <= *target {
                i = mid + 1;
            } else {
                j = mid - 1;
            }
        }

        if i as usize == letters.len() {
            letters[0]
        } else {
            letters[i as usize]
        }
    }
}
