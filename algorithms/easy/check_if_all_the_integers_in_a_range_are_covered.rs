impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut arr = vec![0; 60];
        for r in &ranges {
            for i in r[0]..=r[1] {
                arr[i as usize] = 1;
            }
        }

        for i in left..=right {
            if arr[i as usize] == 0 {
                return false;
            }
        }
        true
    }
}
