fn next(cur: i32, di: i32, len: usize) -> i32 {
    let mut n = cur + di;
    if n < 0 {
        n = len as i32 - 1;
    } else if n == len as i32 {
        n = 0;
    }
    n
}
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![0; code.len()];
        }
        let mut ans = Vec::new();
        let mut x = 1;
        let mut dx = 1;
        let mut y = x + k - 1;
        if k < 0 {
            y = (code.len() as i32) + k;
            dx = -1;
            x = code.len() as i32 - 1
        }
        let mut sum = 0;
        for i in 0..code.len() {
            if i == 0 {
                let mut p = x;
                while p != y {
                    sum += code[p as usize];
                    p = next(p, dx, code.len());
                }
                sum += code[p as usize];
            } else {
                if dx > 0 {
                    sum -= code[x as usize];
                    x = next(x, dx, code.len());
                    y = next(y, dx, code.len());
                    sum += code[y as usize];
                } else {
                    x = next(x, 1, code.len());
                    sum += code[x as usize];
                    sum -= code[y as usize];
                    y = next(y, 1, code.len());
                }
            }

            ans.push(sum);
        }
        ans
    }
}
