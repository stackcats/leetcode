impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut arr = Vec::new();
        let number: Vec<char> = number.chars().collect();
        for (i, c) in number.iter().enumerate() {
            if *c == digit {
                let mut tmp = number.to_vec();
                tmp.remove(i);
                arr.push(tmp.into_iter().collect::<String>());
            }
        }
        arr.sort_by(|a, b| {
            if a.len() == b.len() {
                b.cmp(a)
            } else {
                b.len().cmp(&a.len())
            }
        });
        arr[0].to_string()
    }
}
