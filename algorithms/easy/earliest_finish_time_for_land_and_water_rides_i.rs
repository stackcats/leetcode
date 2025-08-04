use std::iter::zip;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let land = land_start_time
            .iter()
            .cloned()
            .zip(land_duration.iter().cloned())
            .min_by_key(|(s, d)| s + d)
            .map(|(s, d)| s + d)
            .unwrap();

        let water = water_start_time
            .iter()
            .cloned()
            .zip(water_duration.iter().cloned())
            .min_by_key(|(s, d)| s + d)
            .map(|(s, d)| s + d)
            .unwrap();

        let ans1 = zip(land_start_time, land_duration)
            .map(|(s, d)| water.max(s) + d)
            .min()
            .unwrap();

        let ans2 = zip(water_start_time, water_duration)
            .map(|(s, d)| land.max(s) + d)
            .min()
            .unwrap();

        ans1.min(ans2)
    }
}
