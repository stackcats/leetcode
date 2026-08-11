impl Solution {
    pub fn min_initial_strength(monsters: Vec<i32>, boosts: Vec<Vec<i32>>) -> i64 {
        let n = monsters.len();
        let mut diff = vec![0; n + 1];
        for boost in boosts {
            let &[l, r, v] = boost.as_slice() else {
                unreachable!()
            };
            diff[r as usize] += v as i64;
            if (l as usize) > 0 {
                diff[l as usize - 1] -= v as i64;
            }
        }

        let mut ans = 0;
        let mut bonus = 0;
        for (i, m) in monsters.into_iter().enumerate().rev() {
            bonus += diff[i];
            if ans > 0 {
                ans += m as i64;
            } else {
                ans = (m as i64 - bonus).max(0);
            }
        }
        ans
    }
}
