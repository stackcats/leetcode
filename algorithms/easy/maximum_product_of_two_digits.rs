impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut arr = Vec::new();
        while n > 0 {
            arr.push(n % 10);
            n /= 10;
        }

        let mut ans = 0;
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                ans = ans.max(arr[i] * arr[j]);
            }
        }

        ans
    }
}
