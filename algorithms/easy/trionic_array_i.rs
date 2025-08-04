impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut p = 1;
        while p < nums.len() && nums[p] > nums[p - 1] {
            p += 1;
        }

        if p == 1 {
            return false;
        }

        let mut q = p;
        while q < nums.len() && nums[q] < nums[q - 1] {
            q += 1;
        }

        if q == p || q - 1 >= nums.len() - 1 {
            return false;
        }

        let mut k = q;
        while k < nums.len() && nums[k] > nums[k - 1] {
            k += 1;
        }

        k == nums.len()
    }
}
