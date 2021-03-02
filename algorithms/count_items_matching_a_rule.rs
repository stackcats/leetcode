impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut ct = 0;
        for item in &items {
            if rule_key == "type" && item[0] == rule_value {
                ct += 1;
            } else if rule_key == "color" && item[1] == rule_value {
                ct += 1;
            } else if rule_key == "name" && item[2] == rule_value {
                ct += 1;
            }
        }
        ct
    }
}
