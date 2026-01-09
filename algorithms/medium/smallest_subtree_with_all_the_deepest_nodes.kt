/**
 * Example: var ti = TreeNode(5) var v = ti.`val` Definition for a binary tree node. class
 * TreeNode(var `val`: Int) { var left: TreeNode? = null var right: TreeNode? = null }
 */
class Solution {
  fun rec(root: TreeNode?): Pair<TreeNode?, Int> {
    if (root == null) {
      return Pair(null, 0)
    }

    if (root?.left == null && root?.right == null) {
      return Pair(root, 1)
    }

    val (l, dl) = rec(root?.left)
    val (r, dr) = rec(root?.right)

    if (dl == dr) {
      return Pair(root, dl + 1)
    }

    if (dl > dr) {
      return Pair(l, dl + 1)
    }

    return Pair(r, dr + 1)
  }

  fun subtreeWithAllDeepest(root: TreeNode?): TreeNode? {
    val (node, _) = rec(root)
    return node!!
  }
}
