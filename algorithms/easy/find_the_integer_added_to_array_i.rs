impl Solution {
    pub fn added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        let mut m1 = nums1[0];
        let mut m2 = nums2[0];
        for i in 1..nums1.len() {
            m1 = m1.min(nums1[i]);
            m2 = m2.min(nums2[i]);
        }
        m2 - m1
    }
}
