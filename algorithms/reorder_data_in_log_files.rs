use std::cmp::Ordering;

fn is_digit_log(s: String) -> bool {
    let arr: Vec<&str> = s.split_ascii_whitespace().collect();
    arr[1].chars().nth(0).unwrap().is_ascii_digit()
}
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut letter_logs = Vec::new();
        let mut digit_logs = Vec::new();
        for e in &logs {
            if is_digit_log(e.to_string()) {
                digit_logs.push(e.to_string());
            } else {
                letter_logs.push(e.to_string());
            }
        }
        letter_logs.sort_by(|a, b| {
            let v1: Vec<&str> = a.splitn(2, ' ').collect();
            let v2: Vec<&str> = b.splitn(2, ' ').collect();
            if v1[1].cmp(&v2[1]) == Ordering::Equal {
                v1[0].cmp(&v2[0])
            } else {
                v1[1].cmp(&v2[1])
            }
        });
        for e in &digit_logs {
            letter_logs.push(e.to_string());
        }
        letter_logs
    }
}
