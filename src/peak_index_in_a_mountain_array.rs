// https://leetcode.com/problems/peak-index-in-a-mountain-array/description/

impl Solution {
    pub fn peak_index_in_mountain_array(a: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = a.len() - 1;
        while i <= j {
            let mid: usize = i + (j - i) / 2;
            if (a[mid] > a[mid - 1] && a[mid] > a[mid + 1]) {
                return mid as i32;
            } else if (a[mid] < a[mid - 1]) {
                j = mid - 1;
            } else {
                i = mid + 1;
            }
        }
        0
    }
}
