impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut total = 0;
        let mut ans = 0;
        for each in &box_types {
            if total >= truck_size {
                break;
            }
            let n = each[0];
            let x = each[1];
            if total + n <= truck_size {
                total += n;
                ans += n * x;
            } else {
                ans += (truck_size - total) * x;
                total = truck_size;
            }
        }
        ans
    }
}
