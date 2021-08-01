fn dfs(cs: &[char], ndx: usize, curr: &mut String, ans: &mut Vec<String>) {
    if ndx >= cs.len() {
        ans.push(curr.to_string());
        return;
    }
    if cs[ndx].is_digit(10) {
        curr.push(cs[ndx]);
        dfs(cs, ndx + 1, curr, ans);
        return;
    }
    let mut cp = curr.clone();
    cp.push(cs[ndx].to_lowercase().nth(0).unwrap());
    curr.push(cs[ndx].to_uppercase().nth(0).unwrap());
    dfs(cs, ndx + 1, &mut cp, ans);
    dfs(cs, ndx + 1, curr, ans);
}

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        let s: Vec<char> = s.chars().collect();
        dfs(&s, 0, &mut String::new(), &mut ans);
        ans
    }
}
