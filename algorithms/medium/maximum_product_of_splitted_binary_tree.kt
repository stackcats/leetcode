/**
 * Example: var ti = TreeNode(5) var v = ti.`val` Definition for a binary tree node. class
 * TreeNode(var `val`: Int) { var left: TreeNode? = null var right: TreeNode? = null }
 */
class Solution {
  val MOD = 1000000007
  var ans: Long = 0

  fun aux(node: TreeNode?, total: Long): Long {
    node ?: return 0
    val lft = aux(node.left, total)
    val rht = aux(node.right, total)
    val sum = lft + rht + node.`val`.toLong()
    ans = max(ans, (total - sum) * sum)
    return sum
  }

  fun maxProduct(root: TreeNode?): Int {
    val total = aux(root, 0)
    aux(root, total)
    return (ans % MOD).toInt()
  }
}
