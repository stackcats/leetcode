impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let s = s.as_bytes();
        let mut left_map = vec![0; 26];
        let mut right_map = vec![0; 26];
        let mut left = 0;
        let mut right = 0;
        let mut ans = 0;
        for b in s {
            let i = (b - b'a') as usize;
            right_map[i] += 1;
            if right_map[i] == 1 {
                right += 1;
            }
        }
        for b in s {
            let i = (b - b'a') as usize;

            left_map[i] += 1;
            if left_map[i] == 1 {
                left += 1;
            }

            right_map[i] -= 1;
            if right_map[i] == 0 {
                right -= 1;
            }

            if left == right {
                ans += 1;
            }
        }
        ans
    }
}
