// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
void flatten(struct TreeNode *root) {
  if (!root) {
    return;
  }

  flatten(root->right);

  flatten(root->left);

  struct TreeNode *pre = NULL;
  struct TreeNode *iter = root->left;
  while (iter) {
    pre = iter;
    iter = iter->right;
  }

  if (pre) {
    pre->right = root->right;
    root->right = root->left;
  }

  root->left = NULL;
}
