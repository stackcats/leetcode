use std::collections::HashMap;

fn dfs(mem: &mut HashMap<(usize, i32), bool>, stones: &[i32], curr: usize, k: i32) -> bool {
    if curr == stones.len() - 1 {
        return true;
    }

    let key = (curr, k);
    if let Some(v) = mem.get(&key) {
        return *v;
    }

    let mut r = false;
    for i in curr + 1..stones.len() {
        let diff = stones[i] - stones[curr];

        if diff > k + 1 {
            break;
        }

        if diff == k || diff == k - 1 || diff == k + 1 {
            if dfs(mem, stones, i, diff) {
                r = true;
                break;
            }
        }
    }

    mem.insert(key, r);
    r
}

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        if stones[1] - stones[0] > 1 {
            return false;
        }
        let mut mem = HashMap::new();
        dfs(&mut mem, &stones, 1, 1)
    }
}
