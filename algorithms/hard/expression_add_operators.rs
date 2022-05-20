fn dfs(num: &[u8], i: usize, acc:i64, pre: i64, target: i64, exp: &mut Vec<u8>, ans: &mut Vec<String>) {
    if i == num.len() {        
        if acc == target {
            ans.push(String::from_utf8(exp.to_vec()).unwrap());
        }
        return;
    }
    let len = exp.len();
    if i > 0 {
        exp.push(0); // op placeholder
    }
    let mut n = 0;
    for j in i..num.len() {
        if j > i && num[i] == b'0' {
            break;
        }
        n = n * 10 + (num[j] - b'0') as i64;
        exp.push(num[j]);
        if i == 0 {
            dfs(num, j+1, acc + n, n, target, exp, ans);
        } else {
            exp[len] = b'+';
            dfs(num, j+1, acc + n, n, target, exp, ans);
            exp[len] = b'-';
            dfs(num, j+1, acc - n, -n, target, exp, ans);
            exp[len] = b'*';
            dfs(num, j+1, acc - pre + pre * n, pre * n, target, exp, ans);
        }
    }
    exp.split_off(len);
}

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut ans = Vec::new();
        dfs(num.as_bytes(), 0, 0, 0, target as i64, &mut Vec::new(), &mut ans);
        ans
    }
}
