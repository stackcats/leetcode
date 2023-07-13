package leetcode

func twoSum(nums []int, target int) []int {
	m := make(map[int]int)

	for i, v := range nums {
		if _, exists := m[target-v]; exists {
			return []int{m[target-v], i}
		}
		m[v] = i
	}

	return nil
}
