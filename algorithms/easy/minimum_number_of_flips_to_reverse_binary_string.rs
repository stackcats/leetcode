impl Solution {
    pub fn minimum_flips(mut n: i32) -> i32 {
        let mut v = Vec::new();
        while n > 0 {
            v.push(n % 2);
            n /= 2;
        }

        let mut i = 0;
        let mut j = v.len() - 1;

        let mut ans = 0;
        while i < j {
            if v[i] != v[j] {
                ans += 2;
            }
            i += 1;
            j -= 1;
        }

        ans
    }
}
