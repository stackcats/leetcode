fn aux(nums: &Vec<i32>, r: i32, min_odd: Option<i32>) -> bool {
    for &n in nums {
        if n % 2 == r {
            continue;
        }
        if min_odd.is_none() {
            return false;
        }
        if n - min_odd.unwrap() < 1 {
            return false;
        }
    }

    true
}

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let mut min_odd: Option<i32> = None;
        for &n in &nums1 {
            if n % 2 == 1 {
                min_odd = Some(min_odd.map_or(n, |v| v.min(n)));
            }
        }

        aux(&nums1, 0, min_odd) || aux(&nums1, 1, min_odd)
    }
}
