impl Solution {
    pub fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
        let [mut l, mut h] = bounds[0][..] else {
            unreachable!()
        };

        for i in 1..bounds.len() {
            let mut diff = original[i] - original[i - 1];
            l += diff;
            h += diff;
            l = l.max(bounds[i][0]);
            h = h.min(bounds[i][1]);
        }

        0.max(h - l + 1)
    }
}
