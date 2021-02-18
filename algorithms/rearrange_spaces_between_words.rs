impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let words: Vec<&str> = text.split(' ').filter(|w| w.len() > 0).collect();
        let mut num_of_word = words.len();
        if num_of_word == 1 {
            let mut ans = words[0].to_string();
            while ans.len() < text.len() {
                ans.push(' ');
            }
            return ans;
        }
        let mut num_of_space = 0;
        for c in text.chars() {
            if c == ' ' {
                num_of_space += 1;
            }
        }
        if num_of_space == 0 {
            return text;
        }
        let len = num_of_space / (num_of_word - 1);
        let mut spaces = String::new();
        for i in 0..len {
            spaces.push(' ');
        }
        let mut ans = words.join(&spaces);
        while ans.len() < text.len() {
            ans.push(' ');
        }
        ans
    }
}
