// https://leetcode.com/problems/sliding-window-maximum/
use std::collections::LinkedList;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // deque降序存储当前window中的最大值的下标
        let mut deque = LinkedList::new();

        let mut ans = Vec::new();

        for i in 0..nums.len() {
            if let Some(j) = deque.front() {
                // front已经超出window的范围则pop掉
                // 注意如果使用usize类型判断 不能使用 j <= i - k 判断
                if *j + (k as usize) <= i {
                    deque.pop_front();
                }
            }

            while deque.len() > 0 && nums[*deque.back().unwrap()] < nums[i] {
                deque.pop_back();
            }
            deque.push_back(i);

            if i + 1 >= k as usize {
                ans.push(nums[*deque.front().unwrap()]);
            }
        }

        ans
    }
}
