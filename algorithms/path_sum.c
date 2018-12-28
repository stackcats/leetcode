// https://leetcode.com/problems/path-sum/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
bool hasPathSum(struct TreeNode *root, int sum) {
  if (!root) {
    return false;
  }

  if (!root->left && !root->right) {
    return root->val == sum;
  }

  return hasPathSum(root->left, sum - root->val) ||
    hasPathSum(root->right, sum - root->val);
}
