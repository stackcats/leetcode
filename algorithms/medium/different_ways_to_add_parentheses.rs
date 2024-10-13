impl Solution {
    pub fn diff_ways_to_compute(expr: String) -> Vec<i32> {
        if let Ok(n) = expr.parse::<i32>() {
            return vec![n];
        }
        let mut arr = Vec::new();
        for (i, c) in expr.chars().enumerate() {
            if c.is_digit(10) {
                continue;
            }
            let left = Solution::diff_ways_to_compute(expr[..i].to_string());
            let right = Solution::diff_ways_to_compute(expr[i + 1..].to_string());
            for l in &left {
                for r in &right {
                    let n = match c {
                        '+' => l + r,
                        '*' => l * r,
                        '-' => l - r,
                        _ => unreachable!(),
                    };
                    arr.push(n);
                }
            }
        }
        arr
    }
}
