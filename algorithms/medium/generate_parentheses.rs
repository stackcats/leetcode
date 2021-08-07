// https://leetcode.com/problems/generate-parentheses/

fn rec(l: i32, r: i32, s: String, ans: &mut Vec<String>) {
    if l == 0 && r == 0 {
        ans.push(s);
        return;
    }

    if r < l {
        return;
    }

    if l > 0 {
        rec(l - 1, r, s.clone() + "(", ans);
    }

    if r > 0 {
        rec(l, r - 1, s.clone() + ")", ans);
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return Vec::new();
        }

        let mut ans = Vec::new();
        rec(n, n, "".to_string(), &mut ans);

        ans
    }
}
