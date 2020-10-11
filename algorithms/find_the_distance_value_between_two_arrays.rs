impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut ans = 0;
        for n1 in &arr1 {
            let mut flag = true;
            for n2 in &arr2 {
                if (*n1 - *n2).abs() <= d {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans += 1;
            }
        }
        ans
    }
}
