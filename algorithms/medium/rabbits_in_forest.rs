use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();
        let mut ans = 0;
        for n in answers {
            let v = mp.entry(n).or_insert(0);
            *v += 1;
            if *v == n + 1 {
                ans += *v;
                mp.remove(&n);
            }
        }

        for (k, v) in mp {
            ans += k + 1;
        }
        ans
    }
}
