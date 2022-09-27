impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut arr: Vec<_> = names.into_iter().zip(heights.into_iter()).collect();
        arr.sort_by_key(|(_, h)| -h);
        arr.into_iter().map(|(n, _)| n).collect()
    }
}
