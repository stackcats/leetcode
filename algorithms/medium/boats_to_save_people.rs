impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {        
        people.sort();
        let mut i = 0;
        let mut j = people.len() - 1;
        let mut boats = people.len() as i32;
        while i < j {
            if people[i] + people[j] <= limit {
                i += 1;
                boats -= 1
            }
            j -= 1;
        }
        boats
    }
}
