// https://leetcode.com/problems/intersection-of-two-arrays/description/

impl Solution {
    pub fn intersection(nums1: &mut Vec<i32>, nums2: &mut Vec<i32>) -> Vec<i32> {
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

                while i < nums1.len() - 1 && nums1[i] == nums1[i + 1] {
                    i += 1;
                }
                i += 1;
                while j < nums2.len() - 1 && nums2[j] == nums2[j + 1] {
                    j += 1;
                }
                j += 1;
            }
        }

        arr
    }
}
