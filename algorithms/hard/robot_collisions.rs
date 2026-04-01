#[derive(Debug)]
struct Robot {
    position: i32,
    health: i32,
    direction: u8,
    id: usize,
}

impl Robot {
    fn new(position: i32, health: i32, direction: u8, id: usize) -> Self {
        Self {
            position,
            health,
            direction,
            id,
        }
    }
}

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut rs = Vec::with_capacity(positions.len());
        let directions = directions.as_bytes();
        for i in 0..positions.len() {
            let robot = Robot::new(positions[i], healths[i], directions[i], i);
            rs.push(robot);
        }

        rs.sort_by_key(|r| r.position);

        let mut st = Vec::new();

        for (mut r) in rs {
            if st.is_empty() || r.direction == b'R' {
                st.push(r);
                continue;
            }

            while !st.is_empty() {
                let t = st.last_mut().unwrap();
                if t.direction == b'L' {
                    break;
                }

                if t.health < r.health {
                    r.health -= 1;
                    st.pop();
                    continue;
                }

                if t.health > r.health {
                    t.health -= 1;
                } else {
                    st.pop();
                }
                r.health = 0;
                break;
            }

            if r.health > 0 {
                st.push(r);
            }
        }

        st.sort_by_key(|r| r.id);

        st.into_iter().map(|r| r.health).collect()
    }
}
