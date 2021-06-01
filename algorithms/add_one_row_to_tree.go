/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
// https://leetcode.com/problems/add-one-row-to-tree/description/

func addOneRow(root *TreeNode, v int, d int) *TreeNode {
	if root == nil {
		return nil
	}

	if d == 1 {
		t := &TreeNode{Val: v, Left: root}
		return t
	} else if d == 2 {
		left := &TreeNode{Val: v, Left: root.Left}
		root.Left = left
		right := &TreeNode{Val: v, Right: root.Right}
		root.Right = right
	} else if d > 2 {
		root.Left = addOneRow(root.Left, v, d-1)
		root.Right = addOneRow(root.Right, v, d-1)
	}

	return root
}
