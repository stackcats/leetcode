impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let mut st = Vec::new();
        // use stack to track invalid parentheses
        for (i, &c) in s.iter().enumerate() {
             if c == '(' {
                st.push(i);
             } else if c == ')' {
                 if st.is_empty() || s[st[st.len() - 1]] == ')' {
                     st.push(i);
                 } else {
                     st.pop();
                 }
            }
        }

        // update invalid parentheses to space
        for &i in &st {
            s[i] = ' ';
        }
        
        s.into_iter().filter(|c| *c != ' ').collect()
    }
}
