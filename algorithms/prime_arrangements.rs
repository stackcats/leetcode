fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    let end = (n as f64).sqrt().ceil() as i32;
    for i in 2..=end {
        if n % i == 0 {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let mut num_of_primes = 0_u64;
        for i in 1..=n {
            if is_prime(i) {
                num_of_primes += 1;
            }
        }
        let mut ans = 1_u64;
        let mut others = n as u64 - num_of_primes;
        println!("{} {}", num_of_primes, others);
        while others > 1 {
            ans = others * ans % 1000000007;
            others -= 1;
        }
        while num_of_primes > 1 {
            ans = num_of_primes * ans % 1000000007;
            num_of_primes -= 1;
        }
        ans as i32
    }
}
