fn slice(n: i32) -> Vec<i32> {
    let mut v = vec![n];
    let mut r = 10;
    while n >= r {
        let lft = slice(n / r);
        let rht = slice(n % r);
        for l in &lft {
            for r in &rht {
                v.push(l + r);
            }
        }
        r *= 10;
    }
    v
}

fn is_punishment(n: i32) -> bool {
    let v = slice(n * n);
    v.contains(&n)
}

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=n {
            if is_punishment(i) {
                ans += i * i;
            }
        }
        ans
    }
}
