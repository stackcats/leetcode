// https://leetcode.com/problems/invert-binary-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
struct TreeNode *invertTree(struct TreeNode *root) {
  if (!root) {
    return NULL;
  }

  struct TreeNode *t = root->left;
  root->left = invertTree(root->right);
  root->right = invertTree(t);
  return root;
}
