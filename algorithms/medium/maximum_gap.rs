fn radix_sort(nums: &mut [i32]) {
    let mut output = vec![0; nums.len()];
    for shift in (0..32).step_by(8) {
        sort(nums, &mut output, shift);
        nums.copy_from_slice(&output);
    }
}

fn sort(nums: &mut [i32], output: &mut [i32], shift: i32) {
    let mut ct = vec![0; 256];
    for &n in nums.iter() {
        let byte = ((n >> shift) & 0xff) as usize;
        ct[byte] += 1;
    }

    for i in 1..256 {
        ct[i] += ct[i - 1];
    }

    for &n in nums.iter().rev() {
        let byte = ((n >> shift) & 0xff) as usize;
        ct[byte] -= 1;
        output[ct[byte]] = n;
    }
}

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        radix_sort(&mut nums);
        let mut ans = 0;
        for i in 1..nums.len() {
            ans = ans.max(nums[i] - nums[i - 1]);
        }
        ans
    }
}
