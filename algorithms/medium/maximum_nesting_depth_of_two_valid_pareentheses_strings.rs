impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut depthA = 0;
        let mut depthB = 0;
        let mut ans = Vec::new();
        for c in seq.chars() {
            if c == '(' {
                if depthA <= depthB {
                    ans.push(0);
                    depthA += 1;
                } else {
                    ans.push(1);
                    depthB += 1;
                }
            } else {
                if depthA >= depthB {
                    depthA -= 1;
                    ans.push(0);
                } else {
                    depthB -= 1;
                    ans.push(1);
                }
            }
        }

        ans
    }
}
