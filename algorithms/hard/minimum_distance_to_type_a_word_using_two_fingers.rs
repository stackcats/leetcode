use std::collections::HashMap;

fn char_to_position(c: u8) -> (i32, i32) {
    let offset = (c - b'A') as i32;
    (offset / 6, offset % 6)
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    let (x1, y1) = a;
    let (x2, y2) = b;
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn dfs(
    mem: &mut HashMap<(usize, (i32, i32), Option<(i32, i32)>), i32>,
    word: &[u8],
    i: usize,
    fst: (i32, i32),
    snd: Option<(i32, i32)>,
) -> i32 {
    if i == word.len() {
        return 0;
    }
    let key = (i, fst, snd);
    if let Some(v) = mem.get(&key) {
        return *v;
    }

    let t = char_to_position(word[i]);
    let a = distance(fst, t) + dfs(mem, word, i + 1, t, snd);
    let b = if snd.is_none() {
        0
    } else {
        distance(snd.unwrap(), t)
    } + dfs(mem, word, i + 1, fst, Some(t));

    let ans = a.min(b);
    mem.insert(key, ans);
    ans
}

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let word = word.as_bytes();
        let fst = char_to_position(word[0]);
        let mut mem = HashMap::new();
        dfs(&mut mem, word, 1, fst, None)
    }
}
