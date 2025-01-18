fn gcd(a: i32, b: i32) -> i32 {
    let r = a % b;
    if r == 0 {
        b
    } else {
        gcd(b, r)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut p = 1;
            let mut g = 0;
            let mut l = 1;
            for j in i..nums.len() {
                p *= nums[j];
                g = gcd(g, nums[j]);
                l = lcm(l, nums[j]);
                if p == g * l {
                    println!("{} {}", i, j);
                    ans = ans.max(j - i + 1);
                }
            }
        }
        ans as i32
    }
}
