use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq)]
struct MyFloat(f64);

impl Eq for MyFloat {}

impl PartialOrd for MyFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MyFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn calc(p: i32, t: i32) -> MyFloat {
    let p = p as f64;
    let t = t as f64;
    MyFloat((p + 1.) / (t + 1.) - p / t)
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut h = BinaryHeap::new();

        for v in classes {
            let (p, t) = (v[0], v[1]);
            h.push((calc(p, t), p, t));
        }

        for _ in 0..extra_students {
            let (_, p, t) = h.pop().unwrap();
            h.push((calc(p + 1, t + 1), p + 1, t + 1));
        }

        let n = h.len() as f64;
        let mut sum = 0.;
        for (_, p, t) in h {
            sum += (p as f64) / (t as f64);
        }

        sum / n
    }
}
