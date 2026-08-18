fn dfs(
    mem: &mut Vec<Vec<Vec<Option<i32>>>>,
    word1: &[u8],
    i: usize,
    word2: &[u8],
    j: usize,
    target: &[u8],
    t: usize,
) -> i32 {
    if t == target.len() {
        let v = if i > 0 && j > 0 { 1 } else { 0 };
        mem[i][j][t] = Some(v);
        return v;
    }

    if let Some(v) = mem[i][j][t] {
        return v;
    }

    let md = 1000000000 + 7;

    let mut v = 0;
    for x in i..word1.len() {
        if word1[x] == target[t] {
            v = (v + dfs(mem, word1, x + 1, word2, j, target, t + 1)) % md;
        }
    }

    for y in j..word2.len() {
        if word2[y] == target[t] {
            v = (v + dfs(mem, word1, i, word2, y + 1, target, t + 1)) % md;
        }
    }

    mem[i][j][t] = Some(v);
    v
}

impl Solution {
    pub fn interleave_characters(word1: String, word2: String, target: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let target = target.as_bytes();

        let mut mem = vec![vec![vec![None; target.len() + 1]; word2.len() + 1]; word1.len() + 1];
        dfs(&mut mem, &word1, 0, &word2, 0, &target, 0)
    }
}
