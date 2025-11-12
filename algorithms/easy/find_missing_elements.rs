impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut mp = vec![0; 105];
        let mut mi = 200;
        let mut ma = 0;
        for n in nums {
            if mi > n {
                mi = n;
            }
            if ma < n {
                ma = n;
            }
            mp[n as usize] = 1;
        }

        let mut ans = Vec::new();
        for i in mi..=ma {
            if mp[i as usize] == 0 {
                ans.push(i);
            }
        }

        ans
    }
}
