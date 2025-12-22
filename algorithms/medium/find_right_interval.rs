#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Item {
    start: i32,
    end: i32,
    ndx: usize,
}

impl Item {
    fn new(start: i32, end: i32, ndx: usize) -> Self {
        Self { start, end, ndx }
    }
}

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = Vec::new();
        for i in 0..intervals.len() {
            arr.push(Item::new(intervals[i][0], intervals[i][1], i));
        }

        arr.sort();

        let mut ans = vec![-1; intervals.len()];

        for i in 0..arr.len() {
            let Item { start, end, ndx } = arr[i];

            if start == end {
                ans[ndx] = ndx as i32;
                continue;
            }

            let mut l = i + 1;
            let mut r = arr.len() - 1;
            while l < r {
                let mut mid = (l + r) / 2;
                if arr[mid].start >= end {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }

            if end <= arr[r].start {
                ans[ndx] = arr[r].ndx as i32;
            }
        }

        ans
    }
}
