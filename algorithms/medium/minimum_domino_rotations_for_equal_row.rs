pub fn rotate(tops: &[i32], bottoms: &[i32], n: i32) -> i32 {
    let mut top = 0;
    let mut bottom = 0;
    for i in 0..tops.len() {
        if tops[i] != n && bottoms[i] != n {
            return 20000;
        }
        if tops[i] != n {
            top += 1;
        }
        if bottoms[i] != n {
            bottom += 1;
        }
    }
    top.min(bottom)
}

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let a = rotate(&tops, &bottoms, tops[0]);
        let b = rotate(&tops, &bottoms, bottoms[0]);
        let c = a.min(b);
        if c == 20000 {
            return -1;
        }
        c
    }
}
