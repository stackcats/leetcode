use std::collections::HashMap;

fn aux(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => '_',
            _ => c,
        })
        .collect()
}

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut m1 = HashMap::new();
        let mut m2 = HashMap::new();
        let mut m3 = HashMap::new();

        for w in wordlist.into_iter().rev() {
            let l = w.to_lowercase();
            m1.insert(w.to_string(), true);
            m2.insert(l.to_string(), w.to_string());
            let t = aux(&l);
            m3.insert(t, w.to_string());
        }

        queries
            .into_iter()
            .map(|q| {
                let l = q.to_lowercase();
                if m1.contains_key(&q) {
                    q
                } else if let Some(w) = m2.get(&l) {
                    w.to_string()
                } else if let Some(w) = m3.get(&aux(&l)) {
                    w.to_string()
                } else {
                    "".to_string()
                }
            })
            .collect()
    }
}
