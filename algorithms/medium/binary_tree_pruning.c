// https://leetcode.com/problems/binary-tree-pruning/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
struct TreeNode *pruneTree(struct TreeNode *root) {
  if (!root) {
    return NULL;
  }

  root->left = pruneTree(root->left);
  root->right = pruneTree(root->right);
  return !root->left && !root->right && root->val == 0 ? NULL : root;
}
