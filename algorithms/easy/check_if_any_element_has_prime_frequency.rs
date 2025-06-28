fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    let m = (n as f64).sqrt().ceil() as i32;
    for i in 2..=m {
        if n % i == 0 {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut mp = vec![0; 105];
        for n in nums {
            mp[n as usize] += 1;
        }
        for i in 0..=100 {
            if is_prime(mp[i]) {
                return true;
            }
        }
        false
    }
}
