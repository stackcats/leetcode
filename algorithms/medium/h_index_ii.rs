impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = citations.len();
        while l < r {
            let mid = l + (r - l) / 2;
            if citations[mid] >= (citations.len() - mid) as i32 {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        (citations.len() - r) as i32
    }
}
