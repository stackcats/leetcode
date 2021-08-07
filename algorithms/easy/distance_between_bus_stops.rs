impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let all: i32 = distance.iter().sum();
        let mut d = 0;
        let i = start.min(destination);
        let j = start.max(destination);
        for n in i..j {
            d += distance[n as usize];
        }

        d.min(all - d)
    }
}
