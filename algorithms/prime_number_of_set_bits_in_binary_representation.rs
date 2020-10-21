impl Solution {
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let mut ans = 0;
        for i in l..=r {
            let n = Solution::num_bits(i);
            if Solution::is_prime(n) {
                ans += 1;
            }
        }
        ans
    }

    pub fn num_bits(mut i: i32) -> i32 {
        let mut ans = 0;
        while i > 0 {
            if i % 2 == 1 {
                ans += 1;
            }
            i /= 2;
        }
        ans
    }

    pub fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}
