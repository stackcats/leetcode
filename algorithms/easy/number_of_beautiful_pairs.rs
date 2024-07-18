fn gcd(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}

fn first(mut a: i32) -> i32 {
    while a >= 10 {
        a /= 10;
    }
    a
}

impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut ct = 0;
        let mut arr = [0; 10];
        for n in nums {
            for i in 1..10 {
                if gcd(i, n % 10) == 1 {
                    ct += arr[i as usize];
                }
            }
            arr[first(n) as usize] += 1;
        }
        ct
    }
}
