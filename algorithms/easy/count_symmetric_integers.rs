fn is_symmetric(mut n: i32) -> bool {
    let mut arr = Vec::new();
    while n > 0 {
        arr.push(n % 10);
        n /= 10;
    }

    if arr.len() % 2 == 1 {
        return false;
    }

    let left = &arr[..arr.len() / 2].iter().sum::<i32>();
    let right = &arr[arr.len() / 2..].iter().sum::<i32>();
    left == right
}

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut ct = 0;
        for i in low..=high {
            if is_symmetric(i) {
                ct += 1;
            }
        }
        ct
    }
}
