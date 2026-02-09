/**
 * Example: var ti = TreeNode(5) var v = ti.`val` Definition for a binary tree node. class
 * TreeNode(var `val`: Int) { var left: TreeNode? = null var right: TreeNode? = null }
 */
class Solution {
  fun inorder(root: TreeNode?, lst: ArrayList<Int>) {
    if (root == null) {
      return
    }
    inorder(root.left, lst)
    lst.add(root.`val`)
    inorder(root.right, lst)
  }

  fun build(l: Int, r: Int, lst: ArrayList<Int>): TreeNode? {
    if (l > r) {
      return null
    }

    val m = (l + r) / 2
    val node = TreeNode(lst[m])
    node.left = build(l, m - 1, lst)
    node.right = build(m + 1, r, lst)
    return node
  }

  fun balanceBST(root: TreeNode?): TreeNode? {
    val lst = ArrayList<Int>()
    inorder(root, lst)
    return build(0, lst.size - 1, lst)
  }
}
