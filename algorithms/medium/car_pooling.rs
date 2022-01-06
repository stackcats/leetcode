impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, mut capacity: i32) -> bool {
        let mut arr = vec![0; 1001];

        for trip in &trips {
            if let &[n, from, to] = &trip[..3] {
                arr[from as usize] += n;
                arr[to as usize] -= n;
            }
        }

        for &n in &arr {
            if capacity < n {
                return false;
            }
            capacity -= n;
        }

        true
    }
}
