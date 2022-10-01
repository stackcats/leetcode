impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut l = 0;
        let mut r = arr.len() - k;
        while l < r {
            let mid = (l + r) / 2;
            if x - arr[mid] > arr[mid + k] - x {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        arr[l..l + k].to_vec()
    }
}
