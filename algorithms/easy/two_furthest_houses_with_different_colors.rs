impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = colors.len() - 1;
        while colors[i] == colors[colors.len() - 1] {
            i += 1;
        }
        while colors[0] == colors[j] {
            j -= 1;
        }
        j.max(colors.len() - i - 1) as i32
    }
}
