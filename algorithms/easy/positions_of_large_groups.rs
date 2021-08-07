impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let arr: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut start = 0;
        let mut end = 0;
        let mut len = 0;
        while i <= arr.len() {
            if i == 0 {
                start = 0;
                len = 1;
            } else if i < arr.len() {
                if arr[i] == arr[i - 1] {
                    len += 1;
                } else {
                    end = i - 1;
                    if len >= 3 {
                        ans.push(vec![start as i32, end as i32]);
                    }
                    start = i;
                    len = 1;
                }
            } else {
                end = i - 1;
                if len >= 3 {
                    ans.push(vec![start as i32, end as i32]);
                }
            }

            i += 1;
        }
        ans
    }
}
