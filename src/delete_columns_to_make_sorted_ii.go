// https://leetcode.com/problems/delete-columns-to-make-sorted-ii/

func minDeletionSize(A []string) int {
	row := len(A)
	if row <= 1 {
		return 0
	}

	col := len(A[0])

	d := make([]int, row)

	ans := 0

	for j := 0; j < col; j++ {
		sorted := true
		for i := 1; i < row; i++ {
			if d[i] == 1 {
				continue
			}

			if A[i][j] < A[i-1][j] {
				ans++
				sorted = false
				break
			}
		}

		if sorted {
			for i := 1; i < row; i++ {
				if A[i][j] > A[i-1][j] {
					d[i] = 1
				}
			}
		}
	}

	return ans
}
