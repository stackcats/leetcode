use std::collections::HashMap;

fn dfs(
    events: &Vec<Vec<i32>>,
    curr: usize,
    remaining: i32,
    memo: &mut HashMap<(usize, i32), i32>,
) -> i32 {
    if curr >= events.len() || remaining <= 0 {
        return 0;
    }

    let key = (curr, remaining);
    if let Some(val) = memo.get(&key) {
        return *val;
    }

    let mut l = curr + 1;
    let mut r = events.len() - 1;
    while l <= r {
        let mid = l + (r - l) / 2;
        if events[mid][0] > events[curr][1] {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    let ndx = r + 1;

    let a = events[curr][2] + dfs(events, ndx, remaining - 1, memo);
    let b = dfs(events, curr + 1, remaining, memo);

    let val = a.max(b);
    memo.insert(key, val);
    val
}

impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort();
        let mut memo = HashMap::new();
        dfs(&events, 0, 2, &mut memo)
    }
}
