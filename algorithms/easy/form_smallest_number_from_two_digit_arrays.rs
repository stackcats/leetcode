impl Solution {
    pub fn min_number(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        if nums1[0] == nums2[0] {
            nums1[0]
        } else if nums1[0] < nums2[0] {
            nums1 * 10 + nums2[0]
        } else {
            nums2[0] * 10 + nums[1]
        }
    }
}
