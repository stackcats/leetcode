impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let l = nums.len() as i32;
        for i in 0..nums.len() {
            res[i] = nums[(i as i32 + nums[i]).rem_euclid(l) as usize];
        }
        res
    }
}
