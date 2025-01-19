impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let mut arr = vec![0];
        let mut ans = 0;
        for i in 0..nums.len() {
            arr.push(arr[i] + nums[i]);
            let start = (i as i32 - nums[i]).max(0) as usize;
            ans += arr[i + 1] - arr[start];
        }
        ans
    }
}
