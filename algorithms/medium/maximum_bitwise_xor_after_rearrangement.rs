impl Solution {
    pub fn maximum_xor(s: String, t: String) -> String {
        let mut one = 0;
        let mut zero = 0;
        for c in t.chars() {
            if c == '1' {
                one += 1;
            } else {
                zero += 1;
            }
        }

        let mut ans = String::new();

        for c in s.chars() {
            if c == '1' {
                if zero > 0 {
                    zero -= 1;
                    ans.push('1');
                } else {
                    one -= 1;
                    ans.push('0');
                }
            } else {
                if one > 0 {
                    one -= 1;
                    ans.push('1');
                } else {
                    zero -= 1;
                    ans.push('0');
                }
            }
        }

        ans
    }
}
