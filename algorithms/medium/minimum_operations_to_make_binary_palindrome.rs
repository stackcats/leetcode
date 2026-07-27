lazy_static::lazy_static! {
    static ref ps: Vec<i32> = {
        (1..=6000).filter(|n| is_p(*n)).collect()
    };
}

fn is_p(mut n: i32) -> bool {
    let mut v = Vec::new();
    while n > 0 {
        v.push(n % 2);
        n /= 2;
    }
    let mut i = 0;
    let mut j = v.len() - 1;
    while i < j {
        if v[i] != v[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .map(|n| match ps.binary_search(&n) {
                Ok(_) => 0,
                Err(i) => (n - ps[i - 1]).min(ps[i] - n),
            })
            .collect()
    }
}
