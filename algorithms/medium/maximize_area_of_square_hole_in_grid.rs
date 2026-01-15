fn max_gap(mut arr: Vec<i32>) -> i32 {
    arr.sort();
    let mut curr = 1;
    let mut ans = 1;
    for i in 1..arr.len() {
        if arr[i] == arr[i - 1] + 1 {
            curr += 1;
        } else {
            curr = 1;
        }
        ans = ans.max(curr);
    }
    ans + 1
}

impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let w = max_gap(v_bars);
        let h = max_gap(h_bars);
        let s = w.min(h);
        s * s
    }
}
