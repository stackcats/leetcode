fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    let m = (n as f64).sqrt() as i32 + 1;
    for i in 2..=m {
        if n % i == 0 {
            return false;
        }
    }

    true
}

impl Solution {
    pub fn complete_prime(num: i32) -> bool {
        let s = format!("{}", num);
        let mut v = Vec::new();
        for i in 0..s.len() {
            let t = &s[0..=i];
            v.push(t.parse::<i32>().unwrap());
            let t = &s[i..s.len()];
            v.push(t.parse::<i32>().unwrap());
        }

        v.into_iter().all(|n| is_prime(n))
    }
}
