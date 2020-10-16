impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut arr = vec![0; num_people as usize];
        let mut given = 1;
        let mut i = 0;
        while candies > 0 {
            let n = candies.min(given);
            arr[i as usize] += n;
            candies -= n;
            given += 1;
            i += 1;
            i %= num_people;
        }
        arr
    }
}
