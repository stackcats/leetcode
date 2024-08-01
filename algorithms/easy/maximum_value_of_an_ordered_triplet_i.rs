impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut maxi = 0;
        let mut maxij = 0;
        for n in nums {
            let n = n as i64;
            ans = ans.max(maxij * n);
            maxij = maxij.max(maxi - n);
            maxi = maxi.max(n);
        }
        ans
    }
}
