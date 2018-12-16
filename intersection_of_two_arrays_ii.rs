// https://leetcode.com/problems/intersection-of-two-arrays-ii/description/

impl Solution {
    pub fn intersect(nums1: &mut Vec<i32>, nums2: &mut Vec<i32>) -> Vec<i32> {
        nums1.sort();
        nums2.sort();

        let mut i = 0;
        let mut j = 0;

        let mut arr = Vec::new();
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                i += 1;
            } else if (nums1[i] > nums2[j]) {
                j += 1;
            } else {
                arr.push(nums1[i]);
                i += 1;
                j += 1;
            }
        }

        arr
    }
}
