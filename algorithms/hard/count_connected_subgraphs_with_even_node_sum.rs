use std::collections::VecDeque;

fn check(start: usize, mut mask: usize, nums: &Vec<i32>, g: &Vec<Vec<usize>>) -> i32 {
    let mut q = VecDeque::new();
    q.push_back(start);
    mask ^= (1 << start);
    let mut sum = 0;
    while let Some(u) = q.pop_front() {
        sum += nums[u];
        for &v in &g[u] {
            if mask & (1 << v) > 0 {
                mask ^= (1 << v);
                q.push_back(v);
            }
        }
    }

    if mask == 0 && sum % 2 == 0 { 1 } else { 0 }
}

impl Solution {
    pub fn even_sum_subgraphs(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut g = vec![vec![]; n];
        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }

        let mut ans = 0;
        for mask in 1..(1 << n) {
            for i in 0..n {
                if mask & (1 << i) > 0 {
                    ans += check(i, mask, &nums, &g);
                    break;
                }
            }
        }
        ans
    }
}
