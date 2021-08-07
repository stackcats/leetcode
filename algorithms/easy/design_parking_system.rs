struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { big, medium, small }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if car_type == 1 {
            if self.big > 0 {
                self.big -= 1;
                return true;
            }
            return false;
        }
        if car_type == 2 {
            if self.medium > 0 {
                self.medium -= 1;
                return true;
            }
            return false;
        }
        if self.small > 0 {
            self.small -= 1;
            return true;
        }
        false
    }
}
