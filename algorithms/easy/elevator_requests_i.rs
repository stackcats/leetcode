impl Solution {
    pub fn elevator_requests(n: i32, requests: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut pre = 0;
        for r in requests {
            ans += (pre - r).abs();
            pre = r;
        }
        ans
    }
}
