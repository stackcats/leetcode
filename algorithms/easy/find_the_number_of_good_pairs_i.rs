impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut ct = 0;
        for n in &nums1 {
            for m in &nums2 {
                if n % (m * k) == 0 {
                    ct += 1;
                }
            }
        } 
        ct
    }
}
