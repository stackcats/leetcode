// https://leetcode.com/problems/all-possible-full-binary-trees/description/

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func allPossibleFBT(N int) []*TreeNode {
	arr := make([]*TreeNode, 0)

	if N == 1 {
		arr = append(arr, &TreeNode{Val: 0})
	} else if N%2 != 0 {
		for i := 1; i < N; i += 2 {
			for _, left := range allPossibleFBT(i) {
				for _, right := range allPossibleFBT(N - i - 1) {
					t := &TreeNode{Val: 0}
					t.Left = left
					t.Right = right
					arr = append(arr, t)
				}
			}
		}
	}

	return arr
}
