package target_sum

import (
	"testing"
)

func TestFindTargetSumWays(t *testing.T) {
	tests := []struct {
		nums     []int
		target   int
		expected int
	}{
		{[]int{1}, 1, 1},
		{[]int{1, 1, 1, 1, 1}, 3, 5},
		{[]int{22, 36, 7, 44, 38, 32, 16, 32, 1, 16, 25, 45, 49, 45, 27, 9, 41, 31, 10, 15}, 1, 5975},
		{[]int{11, 31, 37, 36, 43, 40, 50, 18, 10, 15, 10, 35, 43, 25, 41, 43, 6, 22, 38, 38}, 44, 5381},
	}

	for _, test := range tests {
		actual := findTargetSumWays(test.nums, test.target)
		if actual != test.expected {
			t.Errorf("target %d of %v is %d, expected %d", test.target, test.nums, actual, test.expected)
		}
	}
}
