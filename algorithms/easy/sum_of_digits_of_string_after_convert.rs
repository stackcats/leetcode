impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut arr: Vec<i32> = s
            .as_bytes()
            .into_iter()
            .map(|b| (b - b'a' + 1) as i32)
            .collect();
        let mut num = 0;
        for i in 0..arr.len() {
            while arr[i] > 0 {
                num += arr[i] % 10;
                arr[i] /= 10;
            }
        }

        for i in 1..k {
            let mut ans = 0;
            while num > 0 {
                ans += num % 10;
                num /= 10;
            }
            num = ans;
        }
        num
    }
}
