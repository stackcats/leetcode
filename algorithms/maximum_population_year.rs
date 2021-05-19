impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut largest = 0;
        let mut ans = 1950;
        let mut arr = vec![0; 2050 - 1950 + 1];
        for i in 0..arr.len() {
            let year = i as i32 + 1950;
            for each in &logs {
                if year >= each[0] && year < each[1] {
                    arr[i] += 1;
                    if largest < arr[i] {
                        largest = arr[i];
                        ans = year;
                    }
                }
            }
        }
        ans
    }
}
