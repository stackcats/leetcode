fn capitalize(s: &str) -> String {
    let s = s.to_lowercase();
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

impl Solution {
    pub fn generate_tag(caption: String) -> String {
        let mut ans = String::new();
        ans.push('#');
        for (i, s) in caption.split_whitespace().enumerate() {
            if i == 0 {
                ans.push_str(s.to_lowercase().as_str());
            } else {
                ans.push_str(capitalize(s).as_str());
            }
        }
        ans.truncate(100);
        ans
    }
}
