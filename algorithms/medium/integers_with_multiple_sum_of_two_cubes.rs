use std::collections::HashMap;

impl Solution {
    pub fn find_good_integers(n: i32) -> Vec<i32> {
        let r = (n as f64).cbrt().ceil() as i32;
        let mut mp = HashMap::new();
        for i in 1..r {
            let a = i * i * i;
            for j in (i + 1)..r {
                let b = j * j * j;
                let x = a + b;
                if x > n {
                    break;
                }
                *mp.entry(x).or_insert(0) += 1;
            }
        }

        let mut ans = Vec::new();
        for (k, v) in mp {
            if v > 1 {
                ans.push(k);
            }
        }

        ans.sort_unstable();

        ans
    }
}
