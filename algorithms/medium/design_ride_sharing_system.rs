use std::collections::{HashSet, VecDeque};

struct RideSharingSystem {
    canceled: HashSet<i32>,
    riders: VecDeque<i32>,
    drivers: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RideSharingSystem {

    fn new() -> Self {
        Self {
            canceled: HashSet::new(),
            riders: VecDeque::new(),
            drivers: VecDeque::new(),
        }
    }
    
    fn add_rider(&mut self, rider_id: i32) {
        self.canceled.remove(&rider_id);
        self.riders.push_back(rider_id);
    }
    
    fn add_driver(&mut self, driver_id: i32) {
        self.drivers.push_back(driver_id);
    }
    
    fn match_driver_with_rider(&mut self) -> Vec<i32> {
        if self.drivers.is_empty() {
            return vec![-1, -1];

        }
       
        while let Some(rider_id) = self.riders.pop_front() {
            if !self.canceled.contains(&rider_id) {
                let driver_id = self.drivers.pop_front().unwrap();
                return vec![driver_id, rider_id];
            }
        }

        vec![-1, -1]
    }
    
    fn cancel_rider(&mut self, rider_id: i32) {
        self.canceled.insert(rider_id);
    }
}

/**
 * Your RideSharingSystem object will be instantiated and called as such:
 * let obj = RideSharingSystem::new();
 * obj.add_rider(riderId);
 * obj.add_driver(driverId);
 * let ret_3: Vec<i32> = obj.match_driver_with_rider();
 * obj.cancel_rider(riderId);
 */
