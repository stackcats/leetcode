impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        costs
            .into_iter()
            .fold((0, 0, 0), |(a, b, c), cost| {
                (b, c, cost + (a + 9).min(b + 4).min(c + 1))
            })
            .2
    }
}
