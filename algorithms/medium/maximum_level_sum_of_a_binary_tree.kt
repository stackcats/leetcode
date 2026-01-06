/**
 * Example: var ti = TreeNode(5) var v = ti.`val` Definition for a binary tree node. class
 * TreeNode(var `val`: Int) { var left: TreeNode? = null var right: TreeNode? = null }
 */
class Solution {
  fun maxLevelSum(root: TreeNode?): Int {
    val q: ArrayDeque<TreeNode> = ArrayDeque()
    var ans = 1
    var level = 1
    var max_sum = root!!.`val`

    q.add(root!!)

    while (q.size > 0) {
      val size = q.size
      var sum = 0
      for (i in 1..size) {
        val node = q.removeFirst()
        sum += node.`val`
        node.left?.let { q.add(node.left) }
        node.right?.let { q.add(node.right) }
      }

      if (max_sum < sum) {
        max_sum = sum
        ans = level
      }
      level += 1
    }

    return ans
  }
}
