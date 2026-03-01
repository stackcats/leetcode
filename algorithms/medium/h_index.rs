impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();

        for (i, c) in citations.iter().enumerate() {
            let h = (citations.len() - i) as i32;
            if *c >= h {
                return h;
            }
        }

        0
    }
}
