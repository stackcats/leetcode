impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let f = *rounds.first().unwrap();
        let e = *rounds.last().unwrap();
        if f <= e {
            return (f..=e).collect();
        }
        (1..=n).filter(|x| x <= &e || x >= &f).collect()
    }
}
