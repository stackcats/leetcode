fn sum(mut n: i32) -> i32 {
    let mut ans = 0;
    while n > 0 {
        ans += n % 10;
        n /= 10;
    }

    ans
}

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for (i, n) in nums.into_iter().enumerate() {
            let i = i as i32;
            if i == sum(n) {
                return i;
            }
        }
        -1
    }
}
