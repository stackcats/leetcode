impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split('-')
            .into_iter()
            .map(|a| format!("{:b}", a.parse::<i32>().unwrap()))
            .collect::<Vec<String>>()
            .join("-")
    }
}
