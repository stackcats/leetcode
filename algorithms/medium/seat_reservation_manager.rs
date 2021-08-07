use std::collections::BinaryHeap;
use std::cmp::Reverse;
    
struct SeatManager {
    seats: BinaryHeap<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {

    fn new(n: i32) -> Self {
        let mut seats = BinaryHeap::new();
        for i in 1..=n {
            seats.push(Reverse(i));
        }
        Self {
            seats
        }
    }
    
    fn reserve(&mut self) -> i32 {
        if let Some(Reverse(n)) = self.seats.pop() {
            return n;
        }
        -1
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.seats.push(Reverse(seat_number));
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */
