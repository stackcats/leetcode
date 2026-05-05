fn reverse(mut n: i32) -> i32 {
    let mut r = 0;
    while n > 0 {
        r = r * 10 + n % 10;
        n /= 10;
    }
    r
}

fn sieve(size: usize) -> Vec<bool> {
    let mut v = vec![true; size];
    v[0] = false;
    v[1] = false;
    for i in 2..size {
        if v[i] {
            let mut k = 2 * i;
            while k < size {
                v[k] = false;
                k += i;
            }
        }
    }
    v
}

impl Solution {
    pub fn sum_of_primes_in_range(n: i32) -> i32 {
        let r = reverse(n);
        let primes = sieve(1001);
        (n.min(r)..=n.max(r)).filter(|&i| primes[i as usize]).sum()
    }
}
