// https://leetcode.com/problems/maximum-depth-of-binary-tree/description/
// 循环的方式同
// https://leetcode.com/problems/maximum-depth-of-n-ary-tree/description/
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
int maxDepth(struct TreeNode *root) {
  if (!root) {
    return 0;
  }

  int left = maxDepth(root->left) + 1;
  int right = maxDepth(root->right) + 1;
  return left < right ? right : left;
}
