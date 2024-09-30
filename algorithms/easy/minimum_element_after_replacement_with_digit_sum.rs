fn aux(mut n: i32) -> i32 {
    let mut m = 0;
    while n > 0 {
        m += n % 10;
        n /= 10;
    }
    m
}

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(aux).min().unwrap()
    }
}
