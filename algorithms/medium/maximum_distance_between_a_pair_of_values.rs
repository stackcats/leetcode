impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 0;

        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if j < i {
                j = i;
                continue;
            }

            if nums1[i] <= nums2[j] {
                ans = ans.max(j - i);
                j += 1;
            } else {
                i += 1;
            }
        }

        ans as _
    }
}
