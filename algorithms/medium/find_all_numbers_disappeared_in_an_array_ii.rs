fn get_ranges(a1: i32, b1: i32, a2: i32, b2: i32, ans: &mut Vec<Vec<i32>>) {
    if a1 > b1 {
        return;
    }

    if a1 <= a2 && a2 <= b1 && b1 <= b2 {
        ans.push(vec![a2, b1]);
    } else if a1 > a2 && b1 < b2 {
        ans.push(vec![a1, b1]);
    } else if a1 <= a2 && b1 >= b2 {
        ans.push(vec![a2, b2]);
    } else if a2 <= a1 && a1 <= b2 && b1 >= b2 {
        ans.push(vec![a1, b2]);
    }
}

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        if nums[0] > upper || nums[nums.len() - 1] < lower {
            return vec![vec![lower, upper]];
        }

        let mut ans: Vec<Vec<i32>> = Vec::new();

        if nums[0] > lower {
            ans.push(vec![lower, nums[0] - 1]);
        }

        for i in 1..nums.len() {
            if nums[i] < lower {
                continue;
            }
            get_ranges(nums[i - 1] + 1, nums[i] - 1, lower, upper, &mut ans);
        }

        if nums[nums.len() - 1] < upper {
            ans.push(vec![nums[nums.len() - 1] + 1, upper]);
        }

        ans
    }
}
