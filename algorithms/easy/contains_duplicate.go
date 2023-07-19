func containsDuplicate(nums []int) bool {
	uniqueNums := make(map[int]bool)

	for _, num := range nums {
		if uniqueNums[num] {
			return true
		} else {
			uniqueNums[num] = true
		}
	}

	return false
}
