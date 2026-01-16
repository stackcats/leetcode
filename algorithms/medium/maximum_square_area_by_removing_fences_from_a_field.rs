use std::collections::HashSet;

fn aux(mut arr: Vec<i32>, m: i32) -> HashSet<i32> {
    arr.push(1);
    arr.push(m);

    let mut st = HashSet::new();

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            let diff = (arr[j] - arr[i]).abs();
            st.insert(diff);
        }
    }

    st
}

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let h = aux(h_fences, m);
        let v = aux(v_fences, n);

        if let Some(&e) = h.intersection(&v).max() {
            let e = e as i64;
            return (e * e % 1000000007) as i32;
        }

        -1
    }
}
