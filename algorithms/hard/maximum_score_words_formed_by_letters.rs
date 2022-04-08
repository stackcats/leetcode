use std::collections::HashMap;

fn can_build_word(s: &str, lettersMap: &HashMap<char, i32>) -> bool {
    let mut ws = HashMap::new();
    for c in s.chars() {
        *ws.entry(c).or_insert(0) += 1;
    }

    for (k, v) in &ws {
        if !lettersMap.contains_key(k) {
            return false;
        }
        if lettersMap[k] < *v {
            return false;
        }
    }
    true
}

fn calc_score(s: &str, score: &[i32]) -> i32 {
    s.chars().fold(0, |acc, c| acc + score[(c as u8 - b'a') as usize])
}

fn update_letters(s: &str, lettersMap: &mut HashMap<char, i32>, added: bool) {
    s.chars().for_each(|c| {
        lettersMap.entry(c).and_modify(|v| {
            if added {
                *v += 1;
            } else {
                *v -= 1;
            }
        });
    });
}

fn dfs(words: &[String], lettersMap: &mut HashMap<char, i32>, score: &[i32], curr: i32, ans: &mut i32) {
    if words.is_empty() {
        if *ans < curr {
            *ans = curr;
        }
        return;
    }

    if can_build_word(&words[0], lettersMap) {
        let s = calc_score(&words[0], score);
        update_letters(&words[0], lettersMap, false);
        dfs(&words[1..], lettersMap, score, curr + s, ans);
        update_letters(&words[0], lettersMap, true);
    }
    
    dfs(&words[1..], lettersMap, score, curr, ans);
}

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut lettersMap = HashMap::new();
        for &c in &letters {
            *lettersMap.entry(c).or_insert(0) += 1;
        }
        let mut ans = 0;
        dfs(&words, &mut lettersMap, &score, 0, &mut ans);
        ans
    }
}
