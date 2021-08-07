impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut ans = 0;
        let mut ones = 0;
        let arr = s.chars().collect::<Vec<char>>();
        for c in &arr {
            if *c == '1' {
                ones += 1;
            }
        }
        let mut zeros = 0;
        for i in 0..(arr.len() - 1) {
            if arr[i] == '0' {
                zeros += 1;
            } else {
                ones -= 1;
            }
            ans = ans.max(zeros + ones);
        }
        ans
    }
}
