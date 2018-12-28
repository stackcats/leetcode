// https://leetcode.com/problems/max-points-on-a-line/

// Definition for a point.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Point {
//   pub x: i32,
//   pub y: i32,
// }
//
// impl Point {
//   #[inline]
//   pub fn new(x: i32, y: i32) -> Self {
//     Point {
//       x,
//       y
//     }
//   }
// }
use std::collections::HashMap;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solution {
    pub fn max_points(points: Vec<Point>) -> i32 {
        if points.len() == 0 {
            return 0;
        }
        let mut m = HashMap::new();
        let mut ans = 0;

        for i in 0..points.len() {
            m.clear();
            let mut same_points = 0;
            let mut max = 0;
            for j in (i + 1)..points.len() {
                let mut x = points[j].x - points[i].x;
                let mut y = points[j].y - points[i].y;
                if x == 0 && y == 0 {
                    same_points += 1;
                    continue;
                }

                let g = gcd(x, y);
                if g != 0 {
                    x /= g;
                    y /= g;
                }

                let ct = m.entry((x, y)).or_insert(0);
                *ct += 1;
                max = max.max(*ct);
            }

            ans = ans.max(max + same_points + 1);
        }

        ans
    }
}
