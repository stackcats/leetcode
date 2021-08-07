use std::collections::HashSet;

enum Direction {
    N,
    S,
    E,
    W,
}

struct Robot {
    x: i32,
    y: i32,
    d: Direction,
    obstacles: HashSet<(i32, i32)>,
}

impl Robot {
    fn new(obstacles: &[Vec<i32>]) -> Self {
        let mut set = HashSet::new();
        for obs in obstacles {
            set.insert((obs[0], obs[1]));
        }
        Robot {
            x: 0,
            y: 0,
            d: Direction::N,
            obstacles: set,
        }
    }

    fn turn_left(&mut self) {
        self.d = match self.d {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        };
    }

    fn turn_right(&mut self) {
        self.d = match self.d {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
    }

    fn go(&mut self) {
        let (x, y) = match self.d {
            Direction::N => (self.x, self.y + 1),
            Direction::W => (self.x - 1, self.y),
            Direction::S => (self.x, self.y - 1),
            Direction::E => (self.x + 1, self.y),
        };
        if self.obstacles.contains(&(x, y)) {
            return;
        }
        self.x = x;
        self.y = y;
    }

    fn dis(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut r = Robot::new(&obstacles);
        let mut ans = 0;
        for c in &commands {
            if *c == -2 {
                r.turn_left();
            } else if *c == -1 {
                r.turn_right();
            } else {
                for i in 0..*c {
                    r.go();
                    ans = ans.max(r.dis());
                }
            }
        }
        ans
    }
}
