fn change(stamp: &[char], target: &mut Vec<char>, i: usize, ans: &mut Vec<i32>) -> bool {
    let mut changed = false;
    for j in 0..stamp.len() {
        if target[i+j] == '?' {
            continue;
        }
        if target[i+j] != stamp[j] {
            return false;
        }
        changed = true;
    }
    if changed {
        for j in 0..stamp.len() {
            target[i+j] = '?';
        }
        ans.push(i as i32);
    }
    changed
}

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp: Vec<_> = stamp.chars().collect();
        let mut target: Vec<_> = target.chars().collect();
        let mut ans = Vec::new();
        loop {
            let mut changed = false;
            for i in 0..=(target.len() - stamp.len()) {
                changed |= change(&stamp, &mut target, i, &mut ans);
            }
            if !changed {
                break;
            }
        }

        if target.iter().all(|x| *x == '?') {
            ans.into_iter().rev().collect()
        } else {
            vec![]
        }
    }
}
