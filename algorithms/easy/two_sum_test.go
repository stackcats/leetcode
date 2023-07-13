package leetcode

import (
	"testing"
)

var tests = [][]int{
	[]int{0, 1},
	[]int{1, 2, 3},
	[]int{2, 7, 11, 15},
}

var targets = []int{
	1,
	4,
	22,
}

var results = [][]int{
	[]int{0, 1},
	[]int{0, 2},
	[]int{1, 3},
}

var caseNum = len(targets)

func TestTwoSum(t *testing.T) {
	for i := 0; i < caseNum; i++ {
		if ret := twoSum(tests[i], targets[i]); ret[0] != results[i][0] || ret[1] != results[i][1] {
			t.Error("test failed")
		} else {
			t.Log("test passed")
		}
	}
}

func BenchmarkTwoSum(b *testing.B) {
	for i := 0; i < caseNum; i++ {
		twoSum(tests[i], targets[i])
	}
}
