impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..rating.len() {
            let mut small_left = 0;
            let mut large_right = 0;
            let mut large_left = 0;
            let mut small_right = 0;
            for j in 0..i {
                if rating[j] < rating[i] {
                    small_left += 1;
                } else {
                    large_left += 1;
                }
            }
            for j in (i + 1)..rating.len() {
                if rating[j] < rating[i] {
                    small_right += 1;
                } else {
                    large_right += 1;
                }
            }
            ans += small_left * large_right;
            ans += large_left * small_right;
        }
        ans
    }
}
