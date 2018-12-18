// https://leetcode.com/problems/counting-bits/description/

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut arr = vec![0; (num + 1) as usize];

        for i in 1..=num {
            let u = i & (i - 1);
            arr[i as usize] = arr[u as usize] + 1;
        }

        arr
    }
}
