// https://leetcode.com/problems/binary-tree-tilt/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
int helper(struct TreeNode *root, int *acc) {
  if (!root) {
    return 0;
  }

  int left = helper(root->left, acc);
  int right = helper(root->right, acc);
  *acc += abs(left - right);
  return left + right + root->val;
}
int findTilt(struct TreeNode *root) {
  int ans = 0;

  helper(root, &ans);
  return ans;
}
