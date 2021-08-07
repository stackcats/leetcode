impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut ans = 0;
        for i in 3..n {
            for j in (i + 1)..n {
                let left = i * i + j * j;
                if left > n * n {
                    break;
                }
                for k in (j + 1)..=n {
                    if left == k * k {
                        ans += 2;
                    }
                }
            }
        }
        ans
    }
}
