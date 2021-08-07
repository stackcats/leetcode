impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max().unwrap();
        candies.iter().map(|candy| *candy + extra_candies >= *max).collect()
    }
}
