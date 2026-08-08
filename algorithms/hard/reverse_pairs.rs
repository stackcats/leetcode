fn merge(nums: &mut [i32], mid: usize, ans: &mut usize) {
    let mut j = mid;
    for i in 0..mid {
        while j < nums.len() && (nums[i] as i64) > (nums[j] as i64 * 2) {
            j += 1;
        }
        *ans += j - mid;
    }

    let mut left = nums[..mid].to_vec();
    let mut l = 0;
    let mut r = mid;
    let mut t = 0;

    while l < left.len() && r < nums.len() {
        if left[l] < nums[r] {
            nums[t] = left[l];
            l += 1;
        } else {
            nums[t] = nums[r];
            r += 1;
        }
        t += 1;
    }

    while l < left.len() {
        nums[t] = left[l];
        l += 1;
        t += 1;
    }
}

fn merge_sort(nums: &mut [i32], ans: &mut usize) {
    if nums.len() < 2 {
        return;
    }

    let m = nums.len() / 2;
    merge_sort(&mut nums[0..m], ans);
    merge_sort(&mut nums[m..], ans);
    merge(nums, m, ans);
}

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        merge_sort(&mut nums, &mut ans);
        ans as _
    }
}
