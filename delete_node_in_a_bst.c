// https://leetcode.com/problems/delete-node-in-a-bst/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
struct TreeNode *deleteNode(struct TreeNode *root, int key) {
  if (!root) {
    return NULL;
  }

  if (root->val < key) {
    root->right = deleteNode(root->right, key);
    return root;
  }

  if (root->val > key) {
    root->left = deleteNode(root->left, key);
    return root;
  }

  if (!root->left) {
    return root->right;
  }

  if (!root->right) {
    return root->left;
  }

  struct TreeNode *iter = root->right;
  while (iter && iter->left) {
    iter = iter->left;
  }
  iter->left = root->left;

  return root->right;
}
