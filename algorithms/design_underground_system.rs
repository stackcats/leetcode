use std::collections::HashMap;

struct UndergroundSystem {
    users: HashMap<i32, (String, i32)>,
    travels: HashMap<(String, String), (i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        Self {
            users: HashMap::new(),
            travels: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.users.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, end_station: String, t: i32) {
        let (start_station, start) = self.users.get(&id).unwrap();
        let r = self.travels.entry((start_station.to_string(), end_station)).or_insert((0, 0));
        r.0 += t - start;
        r.1 += 1;      
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let (d, ct) = self.travels[&(start_station, end_station)];
        d as f64 / ct as f64
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
