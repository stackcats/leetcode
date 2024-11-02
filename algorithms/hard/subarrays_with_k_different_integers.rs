use std::collections::HashMap;

fn aux(nums: &Vec<i32>, mut k: i32) -> i32 {
    let mut mp = HashMap::new();
    let mut ans = 0;
    let mut i = 0;
    for j in 0..nums.len() {
        let v = mp.entry(nums[j]).or_insert(0);
        if *v == 0 {
            k -= 1;
        }
        *v += 1;
        while k < 0 {
            let v = mp.get_mut(&nums[i]).unwrap();
            *v -= 1;
            if *v == 0 {
                k += 1;
            }
            i += 1;
        }
        ans += j - i + 1
    }
    ans as i32
}

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        aux(&nums, k) - aux(&nums, k - 1)
    }
}
