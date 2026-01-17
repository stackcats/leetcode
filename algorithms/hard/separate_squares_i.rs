fn aux(arr: &Vec<Vec<i32>>, top: f64) -> f64 {
    let mut total = 0.;
    for v in arr {
        let (y, l) = (v[1] as f64, v[2] as f64);
        if y > top {
            break;
        }
        let h = l.min(top - y);
        total += l * h;
    }

    total
}

impl Solution {
    pub fn separate_squares(mut squares: Vec<Vec<i32>>) -> f64 {
        squares.sort_by_key(|k| k[1]);

        let mut total_area = 0.;
        let mut l = i32::MAX;
        let mut r = i32::MIN;

        for i in 0..squares.len() {
            total_area += squares[i][2] as f64 * squares[i][2] as f64;
            l = l.min(squares[i][1]);
            r = r.max(squares[i][1] + squares[i][2]);
        }

        let target = total_area / 2.;

        let mut l = l as f64;
        let mut r = r as f64;

        while r - l > 1e-5 {
            let mid = (l + r) / 2.;
            let area = aux(&squares, mid);
            if area < target {
                l = mid;
            } else {
                r = mid;
            }
        }

        l
    }
}
