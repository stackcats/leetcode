use std::collections::{HashMap, HashSet, VecDeque};

fn dfs(
    tree: &HashMap<String, Vec<String>>,
    word: &str,
    begin_word: &str,
    curr: &mut Vec<String>,
    ans: &mut Vec<Vec<String>>,
) {
    curr.insert(0, word.to_string());

    if word == begin_word {
        ans.push(curr.to_vec());
        curr.remove(0);
        return;
    }

    for next_word in tree.get(word).unwrap_or(&vec![]) {
        dfs(tree, next_word, begin_word, curr, ans);
    }
    
    curr.remove(0);
}

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut word_map = HashMap::new();
        for w in &word_list {
            word_map.insert(w, i32::MAX);
        }

        if !word_map.contains_key(&end_word) {
            return vec![];
        }

        word_map.insert(&begin_word, 0);
        let mut q = VecDeque::new();
        q.push_back(begin_word.to_string());
        let mut paths = HashMap::new();
        let mut min = i32::MAX;
        while !q.is_empty() {
            let word = q.pop_front().unwrap();
            let step = word_map[&word] + 1;
            if step > min {
                break;
            }
            for i in 0..word.len() {
                let mut new_word = word.clone();
                let mut bs = unsafe { new_word.as_bytes_mut() };
                for c in b'a'..=b'z' {
                    bs[i] = c;
                    let t = String::from_utf8_lossy(bs).to_string();
                    if let Some(ct) = word_map.get_mut(&t) {
                        if *ct < step {
                            continue;
                        }
                        if step < *ct {
                            *ct = step;
                            q.push_back(t.to_string());
                        }
                        if t == end_word {
                            min = step;
                        }
                        paths
                            .entry(t.to_string())
                            .or_insert(vec![])
                            .push(word.to_string());
                    }
                }
            }
        }
        let mut ans = Vec::new();
        dfs(&paths, &end_word, &begin_word, &mut vec![], &mut ans);
        ans
    }
}
