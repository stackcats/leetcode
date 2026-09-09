impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut v: Vec<_> = num.to_string().bytes().map(|c| (c - b'0') as i32).collect();

        let mut mp = vec![0; 10];
        for i in 0..v.len() {
            mp[v[i] as usize] = i;
        }

        for i in 0..v.len() {
            for j in (v[i] + 1..10).rev() {
                if mp[j as usize] > i {
                    v.swap(i, mp[j as usize]);
                    return v.into_iter().fold(0, |acc, n| acc * 10 + n);
                }
            }
        }

        num
    }
}
