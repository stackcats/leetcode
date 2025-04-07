fn is_sorted(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        if v[i - 1] > v[i] {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut ct = 0;
        while !is_sorted(&nums) {
            let mut sum = i32::MAX;
            let mut j = 0;
            for i in 1..nums.len() {
                if nums[i] + nums[i - 1] < sum {
                    sum = nums[i] + nums[i - 1];
                    j = i;
                }
            }
            nums[j - 1] += nums[j];
            nums.remove(j);
            ct += 1;
        }
        ct
    }
}
