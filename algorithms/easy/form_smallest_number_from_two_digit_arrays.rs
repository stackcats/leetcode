impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut arr = vec![0; 10];

        let mut a = 10;
        for n in nums1 {
            arr[n as usize] += 1;
            a = a.min(n);
        }

        let mut b = 10;
        for n in nums2 {
            arr[n as usize] += 1;
            b = b.min(n);
        }

        for (i, ct) in arr.into_iter().enumerate() {
            if ct == 2 {
                return i as i32;
            }
        }

        a.min(b) * 10 + a.max(b)
    }
}
