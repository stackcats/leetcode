package target_sum

func findTargetSumWays(nums []int, target int) int {
	var count int
	var backTrace func(int, int)
	backTrace = func(index, sum int) {
		if index == len(nums) {
			if sum == target {
				count++
			}
			return
		}

		backTrace(index+1, sum+nums[index])
		backTrace(index+1, sum-nums[index])
	}

	backTrace(0, 0)
	return count
}
