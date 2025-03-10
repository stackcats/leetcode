impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut ans = 0;
        'outter: for f in fruits {
            for i in 0..baskets.len() {
                if baskets[i] >= f {
                    baskets[i] = 0;
                    continue 'outter;
                }
            }
            ans += 1;
        }
        ans
    }
}
