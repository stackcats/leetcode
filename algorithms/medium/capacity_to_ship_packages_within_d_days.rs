impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let (mut l, mut r) = weights.iter().fold((0, 0), |(l, r), &w| (l.max(w), r + w));

        while l < r {
            let mid = (l + r) / 2;
            let mut d = 1;
            let mut cur = 0;
            for &w in &weights {
                if cur + w > mid {
                    d += 1;
                    cur = 0;
                }
                cur += w;
            }
            if d > days {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }
}
