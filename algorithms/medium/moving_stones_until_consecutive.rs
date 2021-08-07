impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut arr = [a, b, c];
        arr.sort();
        let [a, b, c] = arr;
        let mut mn = 2;
        let mut mx = 0;
        if b - a == 1 && c - b == 1 {
            mn = 0;
        } else if b - a <= 2 || c - b <= 2 {
            mn = 1;
        }
        mx = c - a - 2;
        vec![mn, mx]
    }
}
