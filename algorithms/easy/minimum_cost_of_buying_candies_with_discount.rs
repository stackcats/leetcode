impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_by(|a, b| b.partial_cmp(a).unwrap());
        cost.chunks(3)
            .map(|chunk| chunk.into_iter().take(2).sum::<i32>())
            .sum()
    }
}
