fn count(mut n: i32) -> i32 {
    let mut arr = Vec::new();
    while n > 0 {
        arr.push(n % 10);
        n /= 10;
    }
    let mut ct = 0;
    for i in 1..arr.len() - 1 {
        if arr[i] > arr[i - 1] && arr[i] > arr[i + 1] {
            ct += 1;
        } else if arr[i] < arr[i - 1] && arr[i] < arr[i + 1] {
            ct += 1;
        }
    }
    ct
}

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        let mut ans = 0;
        for n in num1..=num2 {
            ans += count(n);
        }
        ans
    }
}
