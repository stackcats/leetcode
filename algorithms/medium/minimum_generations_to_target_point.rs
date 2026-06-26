use std::collections::HashSet;

#[inline]
fn encode(p: Vec<i32>) -> i32 {
    100 * p[0] + 10 * p[1] + p[2]
}

#[inline]
fn half(a: i32, b: i32) -> i32 {
    let (a1, a2, a3) = (a / 100, a / 10 % 10, a % 10);
    let (b1, b2, b3) = (b / 100, b / 10 % 10, b % 10);
    let c1 = ((a1 + b1) as f64 / 2.0).floor() as i32;
    let c2 = ((a2 + b2) as f64 / 2.0).floor() as i32;
    let c3 = ((a3 + b3) as f64 / 2.0).floor() as i32;
    c1 * 100 + c2 * 10 + c3
}

impl Solution {
    pub fn min_generations(points: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
        if points.len() == 1 && points[0] != target {
            return -1;
        }

        let target = encode(target);

        let mut st = points.into_iter().map(encode).collect::<HashSet<_>>();

        let mut k = 0;
        while !st.contains(&target) {
            let mut tmp = HashSet::new();
            let arr = st.iter().copied().collect::<Vec<_>>();
            for i in 0..arr.len() {
                for j in i + 1..arr.len() {
                    let (a, b) = (arr[i], arr[j]);

                    let c = half(a, b);

                    if st.contains(&c) {
                        continue;
                    }

                    tmp.insert(c);
                }
            }

            if tmp.is_empty() {
                return -1;
            }

            k += 1;
            st.extend(tmp);
        }

        k
    }
}
