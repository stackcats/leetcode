impl Solution {
    pub fn get_least_frequent_digit(mut n: i32) -> i32 {
        let mut mp = vec![0; 10];
        while n > 0 {
            mp[(n % 10) as usize] += 1;
            n /= 10;
        }

        let mut ans = 0;
        let mut ct = i32::MAX;
        for (i, n) in mp.into_iter().enumerate() {
            if n > 0 && n < ct {
                ans = i as i32;
                ct = n;
            }
        }

        ans
    }
}
