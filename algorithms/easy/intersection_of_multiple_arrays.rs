impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = vec![0; 10001];
        let mut N = nums.len() as i32;
        for ns in &nums {
            ns.iter().for_each(|n| arr[*n as usize] += 1);
        }
        arr.into_iter()
            .enumerate()
            .filter(|(_i, ct)| *ct == N)
            .map(|(i, _ct)| i as i32)
            .collect()
    }
}
