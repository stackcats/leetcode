impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        if changed.len() % 2 != 0 {
            return vec![];
        }

        let mut map = vec![0; 100001];
        changed.iter().for_each(|&n| map[n as usize] += 1);

        let mut ans = Vec::with_capacity(changed.len() / 2);

        for i in 0..100001 {
            if map[i] == 0 {
                continue;
            }
            if i * 2 >= map.len() || map[i * 2] < map[i] {
                return vec![];
            }

            while map[i] > 0 {
                ans.push(i as i32);
                map[i] -= 1;
                map[2 * i] -= 1;
            }
        }

        ans
    }
}
