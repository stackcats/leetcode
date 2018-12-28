// https://leetcode.com/problems/balanced-binary-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

bool helper(struct TreeNode *root, int *height) {
  if (!root) {
    *height = 0;
    return true;
  }

  int lh, rh;
  bool left = helper(root->left, &lh);
  bool right = helper(root->right, &rh);
  if (!left || !right) {
    return false;
  }

  if (abs(lh - rh) > 1) {
    return false;
  }

  *height = lh < rh ? rh + 1 : lh + 1;
  return true;
}

bool isBalanced(struct TreeNode *root) {
  if (!root) {
    return true;
  }

  int h = 0;
  return helper(root, &h);
}
