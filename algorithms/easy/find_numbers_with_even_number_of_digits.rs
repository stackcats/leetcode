impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| {
            acc + if x.to_string().len() % 2 == 0 { 1 } else { 0 }
        })
    }
}
