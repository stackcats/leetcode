// https://leetcode.com/problems/increasing-order-search-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
struct TreeNode *increasingBST(struct TreeNode *root) {
  if (!root) {
    return NULL;
  }

  root->right = increasingBST(root->right);
  struct TreeNode *left = increasingBST(root->left);
  root->left = NULL;
  if (!left) {
    return root;
  }

  struct TreeNode *iter = left;
  while (iter && iter->right) {
    iter = iter->right;
  }

  iter->right = root;

  return left;
}
