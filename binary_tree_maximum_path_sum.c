// https://leetcode.com/problems/binary-tree-maximum-path-sum/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

// 不能用宏函数 会重复递归调用
inline int max(a, b) { return a > b ? a : b; }

// m: 全局最优解
// ret: 局部最优解
int helper(struct TreeNode *root, int *m) {
  if (!root) {
    return 0;
  }

  // 只需要和大于0的子树
  int left = max(helper(root->left, m), 0);
  int right = max(helper(root->right, m), 0);

  // 更新全局最优解
  *m = max(*m, left + right + root->val);

  // 局部最优解只能选左右子树之一 因为路径只有一条
  return max(left, right) + root->val;
}

int maxPathSum(struct TreeNode *root) {
  int a = INT_MIN;
  helper(root, &a);
  return a;
}
