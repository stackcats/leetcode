/**
 * Example: var ti = TreeNode(5) var v = ti.`val` Definition for a binary tree node. class
 * TreeNode(var `val`: Int) { var left: TreeNode? = null var right: TreeNode? = null }
 */
class Solution {
  fun depth(root: TreeNode?): Int {
    if (root == null) {
      return 0
    }

    val left = depth(root?.left)
    val right = depth(root?.right)
    if (left == -1 || right == -1) {
      return -1
    }

    if (abs(left - right) < 2) {
      return max(left, right) + 1
    }

    return -1
  }

  fun isBalanced(root: TreeNode?): Boolean {
    return depth(root) != -1
  }
}
