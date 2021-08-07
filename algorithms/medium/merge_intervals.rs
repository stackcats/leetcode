use std::collections::VecDeque;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut ans = Vec::new();
        let mut intervals: VecDeque<Vec<i32>> = intervals.into_iter().collect();
        while intervals.len() > 0 {
            let right = intervals.pop_front().unwrap();
            if ans.len() == 0 {
                ans.push(right);
                continue;
            }

            let left = ans.pop().unwrap();

            if left[1] >= right[0] && left[1] <= right[1] {
                intervals.push_front(vec![left[0].min(right[0]), left[1].max(right[1])]);
                continue;
            }
            if right[1] >= left[0] && right[1] <= left[1] {
                intervals.push_front(vec![left[0].min(right[0]), left[1].max(right[1])]);
                continue;
            }
            ans.push(left);
            ans.push(right);
        }
        for each in &intervals {
            if ans.len() == 0 {
                ans.push(each.clone());
            }
        }

        ans
    }
}
