const N: usize = 100100;

lazy_static::lazy_static! {
    static ref PRIMES: Vec<bool> = {
        let mut v = vec![true; N];
        v[0] = false;
        v[1] = false;
        for i in 2..N {
            if !v[i] {
                continue;
            }
            for j in ((i + i)..N).step_by(i) {
                v[j] = false;
            }
        }

        v
    };
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut is_prime = true;
        let mut ans = 0;
        for n in nums {
            let mut n = n as usize;
            while PRIMES[n] != is_prime {
                ans += 1;
                n += 1;
            }
            is_prime = !is_prime;
        }
        ans as _
    }
}
