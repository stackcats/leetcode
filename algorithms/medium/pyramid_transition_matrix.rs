use std::collections::HashMap;

fn dfs(arr: Vec<char>, mp: &HashMap<(char, char), Vec<char>>) -> bool {
    if arr.len() == 1 {
        return true;
    }

    let mut row: Vec<Vec<char>> = vec![vec![]];

    for i in 1..arr.len() {
        let key = (arr[i - 1], arr[i]);
        if let Some(v) = mp.get(&key) {
            let mut tmp: Vec<Vec<char>> = Vec::new();
            for j in 0..row.len() {
                for c in v {
                    let mut x = row[j].to_vec();
                    x.push(*c);
                    tmp.push(x);
                }
            }
            row = tmp;
        } else {
            return false;
        }
    }

    row.into_iter().any(|v| dfs(v, mp))
}

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut mp = HashMap::new();
        for v in allowed {
            let arr: Vec<char> = v.chars().collect();
            mp.entry((arr[0], arr[1])).or_insert(vec![]).push(arr[2]);
        }

        let cs: Vec<char> = bottom.chars().collect();

        dfs(cs, &mp)
    }
}
