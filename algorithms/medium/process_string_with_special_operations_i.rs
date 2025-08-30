impl Solution {
    pub fn process_str(s: String) -> String {
        let mut t = String::new();
        for c in s.chars() {
            match c {
                '*' => {
                    t.pop();
                }
                '#' => t.push_str(&t.to_string()),
                '%' => t = t.chars().rev().collect::<String>(),
                _ => t.push(c),
            }
        }
        t
    }
}
