fn f(words: &Vec<String>, max_width: usize) -> String {
    let words_len = words.iter().map(|s| s.len()).sum::<usize>();
    let mut spaces = max_width - words_len;
    let mut holes = words.len() - 1;
    let mut s = String::new();
    for i in 0..words.len() {
        s.push_str(&words[i]);
        if i < words.len() - 1 {
            let d = ((spaces as f64) / (holes as f64)).ceil() as usize;
            spaces -= d;
            s.push_str(&format!("{:w$}", " ", w = d))
        }
        holes -= 1;
    }

    format!("{:w$}", s, w = max_width)
}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut arr = Vec::new();
        let mut curr_width = 0;
        let mut curr = Vec::new();
        for w in words {
            let spaces = if curr.is_empty() { 0 } else { curr.len() };
            if curr_width + spaces + w.len() > max_width {
                arr.push(curr);
                curr = vec![];
                curr_width = 0;
            }

            curr_width += w.len();
            curr.push(w);
        }

        if !curr.is_empty() {
            arr.push(curr);
        }

        let mut ans = Vec::new();
        for i in 0..arr.len() {
            if i == arr.len() - 1 {
                ans.push(format!("{:w$}", arr[i].join(" "), w = max_width));
            } else {
                ans.push(f(&arr[i], max_width));
            }
        }

        ans
    }
}
