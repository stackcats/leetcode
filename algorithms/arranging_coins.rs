impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut l = 1;
        let mut r = n as i64;
        while l <= r {
            let mid = l + (r - l) / 2;
            let all: i64 = (1 + mid) * mid / 2;
            if all == n as i64 {
                return mid as i32;
            }
            if all > n as i64 {
                if all - mid <= n as i64 {
                    return (mid - 1) as i32;
                } else {
                    r = mid - 1;
                }
            } else {
                if all + mid + 1 > n as i64 {
                    return mid as i32;
                } else {
                    l = mid + 1;
                }
            }
        }
        0
    }
}
