fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    for i in 2..((n as f64).sqrt() as i32 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let mut ps = Vec::new();
        let col = nums[0].len();
        for i in 0..nums.len() {
            ps.push(nums[i][i]);
            ps.push(nums[i][col - i - 1]);
        }

        ps.sort_by(|a, b| b.partial_cmp(a).unwrap());

        for p in ps {
            if is_prime(p) {
                return p;
            }
        }
        0
    }
}
