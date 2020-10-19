impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut steps = 0;
        for op in &logs {
            if op == "../" {
                steps -= 1;
                if steps < 0 {
                    steps = 0;
                }
            } else if op == "./" {
                continue;
            } else {
                steps += 1;
            }
        }
        steps
    }
}
