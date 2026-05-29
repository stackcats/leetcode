impl Solution {
    pub fn score_validator(events: Vec<String>) -> Vec<i32> {
        let (mut score, mut ct) = (0, 0);
        for event in events {
            match event.as_str() {
                "W" => ct += 1,
                "WD" => score += 1,
                "NB" => score += 1,
                _ => score += event.parse::<i32>().unwrap(),
            }
            if ct >= 10 {
                break;
            }
        }
        vec![score, ct]
    }
}
