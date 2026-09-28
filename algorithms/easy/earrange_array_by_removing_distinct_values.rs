impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let ma = nums.iter().max().unwrap();
        let mut ct = vec![0; *ma as usize + 1];
        let size = nums.len();

        for n in nums {
            ct[n as usize] += 1;
        }

        let mut ans = Vec::new();

        while ans.len() < size {
            for i in 0..ct.len() {
                if ct[i] > 0 {
                    ans.push(i as i32);
                    ct[i] -= 1;
                }
            }
        }

        ans
    }
}
