fn h(n: usize, first_row: bool) -> String {
    if first_row {
        ".".repeat(n)
    } else {
        format!("{}{}", "#".repeat(n - 1), ".")
    }
}

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        for i in 0..m {
            ans.push(h(n as usize, i == 0));
        }
        ans
    }
}
