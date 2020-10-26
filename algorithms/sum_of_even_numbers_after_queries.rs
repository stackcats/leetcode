impl Solution {
    pub fn sum_even_after_queries(mut a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = a.iter().filter(|n| *n % 2 == 0).sum();
        let mut ans = Vec::new();
        for q in &queries {
            let x = q[0];
            let index = q[1] as usize;
            if x % 2 != 0 && a[index] % 2 != 0 {
                sum += x + a[index];
            } else if x % 2 != 0 && a[index] % 2 == 0 {
                sum -= a[index];
            } else if x % 2 == 0 && a[index] % 2 == 0 {
                sum += x;
            }
            ans.push(sum);
            a[index] += x;
        }
        ans
    }
}
