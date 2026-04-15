fn dfs(mem: &mut Vec<Vec<i64>>, rs: &Vec<i32>, fs: &Vec<i32>, r: usize, f: usize) -> i64 {
    if r == rs.len() {
        return 0;
    }

    if f == fs.len() {
        return i64::MAX / 10;
    }

    if mem[r][f] != -1 {
        return mem[r][f];
    }

    let a = dfs(mem, rs, fs, r + 1, f + 1) + (rs[r] - fs[f]).abs() as i64;
    let b = dfs(mem, rs, fs, r, f + 1);
    mem[r][f] = a.min(b);
    mem[r][f]
}

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        robot.sort();

        let mut fs = Vec::new();
        for f in factory {
            let (p, l) = (f[0], f[1]);
            for _ in 0..l {
                fs.push(p);
            }
        }

        fs.sort();

        let mut mem = vec![vec![-1; fs.len()]; robot.len()];

        dfs(&mut mem, &robot, &fs, 0, 0)
    }
}
