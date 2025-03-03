fn dfs(nums: &Vec<i32>, a: usize, b: usize, mem: &mut Vec<Vec<Option<i32>>>) -> i32 {
    if b == nums.len() {
        return nums[a];
    }

    if b == nums.len() - 1 {
        return nums[a].max(nums[b]);
    }

    if let Some(cost) = mem[a][b] {
        return cost;
    }

    let mut cost = nums[a].max(nums[b]) + dfs(nums, b + 1, b + 2, mem);
    cost = cost.min(nums[a].max(nums[b + 1]) + dfs(nums, b, b + 2, mem));
    cost = cost.min(nums[b].max(nums[b + 1]) + dfs(nums, a, b + 2, mem));

    mem[a][b] = Some(cost);

    cost
}

impl Solution {
    pub fn min_cost(nums: Vec<i32>) -> i32 {
        let mut mem = vec![vec![None; nums.len()]; nums.len()];
        dfs(&nums, 0, 1, &mut mem)
    }
}
