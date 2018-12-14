// https://leetcode.com/problems/find-peak-element/description/

impl Solution {
    pub fn find_peak_element(a: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = a.len() - 1;
        let mut mid: usize = 0;

        while i <= j {
            mid = i + (j - i) / 2;
            if mid > 0 && a[mid] < a[mid - 1] {
                j = mid - 1;
            } else if mid + 1 < a.len() && a[mid] < a[mid + 1] {
                i = mid + 1;
            } else {
                break;
            }
        }
        mid as i32
    }
}
