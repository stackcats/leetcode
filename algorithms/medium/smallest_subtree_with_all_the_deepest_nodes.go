// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/description/

/**
* Definition for a binary tree node.
	* type TreeNode struct {
	*     Val int
	*     Left *TreeNode
	*     Right *TreeNode
	* }
*/
func rec(root *TreeNode) (*TreeNode, int) {
	if root == nil {
		return nil, 0
	}

	if root.Left == nil && root.Right == nil {
		return root, 1
	}

	l, dl := rec(root.Left)
	r, dr := rec(root.Right)

	if dl > dr {
		return l, dl + 1
	} else if dl < dr {
		return r, dr + 1
	} else {
		return root, dl + 1
	}
}

func subtreeWithAllDeepest(root *TreeNode) *TreeNode {
	t, _ := rec(root)
	return t
}
