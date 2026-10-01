use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let mut mp = HashMap::new();
        for (mut each) in knowledge {
            let v = each.pop().unwrap();
            let k = each.pop().unwrap();
            mp.insert(k, v);
        }

        let mut t = String::new();
        let mut i = 0;
        let s = s.as_bytes();
        while i < s.len() {
            if s[i] == b'(' {
                i += 1;
                let mut k = String::new();
                while s[i] != b')' {
                    k.push(s[i] as char);
                    i += 1;
                }
                if let Some(v) = mp.get(&k) {
                    t.push_str(v);
                } else {
                    t.push('?');
                }
            } else {
                t.push(s[i] as char);
            }

            i += 1;
        }

        t
    }
}
