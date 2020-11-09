use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let mut map = HashMap::new();
        let mut start = HashMap::new();
        let mut end = HashMap::new();
        let mut degree = 0;
        let mut arr = Vec::new();

        for i in 0..nums.len() {
            let counter = map.entry(nums[i]).or_insert(0);
            *counter += 1;
            if *counter == 1 {
                start.insert(nums[i], i);
            } else {
                end.insert(nums[i], i);
            }
            if degree < *counter {
                degree = *counter;
                arr = vec![nums[i]];
            } else if degree == *counter {
                arr.push(nums[i]);
            }
        }
        if degree == 1 {
            return 1;
        }
        let mut ans = nums.len();
        for n in &arr {
            let len = *end.entry(*n).or_insert(start[n]) - start[n] + 1;
            if ans > len {
                ans = len;
            }
        }
        ans as i32
    }
}
