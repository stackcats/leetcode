fn dfs(curr: &mut Vec<char>, n: i32, ans: &mut Vec<String>) {
    if curr.len() as i32 == n {
        ans.push(curr.iter().collect());
        return;
    }

    curr.push('1');
    dfs(curr, n, ans);
    curr.pop();

    if curr.is_empty() || *curr.last().unwrap() == '1' {
        curr.push('0');
        dfs(curr, n, ans);
        curr.pop();
    }
}

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        dfs(&mut vec![], n, &mut ans);
        ans
    }
}
