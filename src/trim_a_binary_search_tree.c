// https://leetcode.com/problems/trim-a-binary-search-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
struct TreeNode *trimBST(struct TreeNode *root, int L, int R) {
  if (!root) {
    return NULL;
  }

  if (root->val < L) {
    return trimBST(root->right, L, R);
  }

  if (root->val > R) {
    return trimBST(root->left, L, R);
  }

  root->left = trimBST(root->left, L, R);
  root->right = trimBST(root->right, L, R);

  return root;
}
