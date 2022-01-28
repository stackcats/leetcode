impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut max_defense = 0;
        let mut ans = 0;
        for p in properties.iter().rev() {
            if p[1] < max_defense {
                ans += 1;
            } else {
                max_defense = p[1];
            }
        }
        ans
    }
}
