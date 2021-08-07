impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut largest = 0;
        for r in &rectangles {
            let n = r[0].min(r[1]);
            if largest < n {
                largest = n;
                ans = 1;
            } else if largest == n {
                ans += 1;
            }
        }
        ans
    }
}
