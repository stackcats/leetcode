impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        for s in commands {
            match s.as_str() {
                "UP" => i -= 1,
                "DOWN" => i += 1,
                "LEFT" => j -= 1,
                "RIGHT" => j += 1,
                _ => unreachable!(),
            }
        }
        i * n + j
    }
}
