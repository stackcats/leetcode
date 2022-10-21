impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            nums[i] = sum;
        }

        let mut ans = Vec::new();
        for query in queries {
            let i = nums.partition_point(|&x| x <= query);
            ans.push(i as i32 + 1);
        }
        ans
    }
}
