fn signFunc(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    if x > 0 {
        return 1;
    }
    -1
}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        signFunc(nums.iter().fold(1, |acc, x| acc * signFunc(*x)))
    }
}
