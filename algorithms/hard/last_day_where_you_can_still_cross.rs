use std::collections::{HashSet, VecDeque};

fn can_cross(row: i32, col: i32, m: usize, cells: &Vec<Vec<i32>>) -> bool {
    let st = &cells[0..m]
        .iter()
        .map(|c| (c[0], c[1]))
        .collect::<HashSet<(i32, i32)>>();

    let mut seen = HashSet::new();

    let mut q: VecDeque<_> = (1..=col)
        .map(|c| (1, c))
        .filter(|v| !st.contains(v))
        .collect();

    while let Some((i, j)) = q.pop_front() {
        if i == row {
            return true;
        }

        if seen.contains(&(i, j)) {
            continue;
        }

        seen.insert((i, j));

        for (di, dj) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let (ni, nj) = (i + di, j + dj);
            if ni < 1 || ni > row || nj < 1 || nj > col || st.contains(&(ni, nj)) {
                continue;
            }
            q.push_back((ni, nj));
        }
    }

    false
}

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut l = 0;
        let mut r = cells.len() - 1;
        while l <= r {
            let m = l + (r - l) / 2;
            if can_cross(row, col, m, &cells) {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        (l - 1) as _
    }
}
