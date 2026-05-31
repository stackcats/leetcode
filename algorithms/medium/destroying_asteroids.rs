impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        let mut mass = mass as i64;
        asteroids.sort();
        for n in asteroids {
            let n = n as i64;
            if mass >= n {
                mass += n;
            } else {
                return false;
            }
        }
        true
    }
}
