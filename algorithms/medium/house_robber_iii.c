// https://leetcode.com/problems/house-robber-iii/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

#define max(a, b) ((a) < (b) ? (b) : (a))

void helper(struct TreeNode *root, int *m, int *n) {
  if (!root) {
    *m = 0;
    *n = 0;
    return;
  }

  int lm, ln;
  helper(root->left, &lm, &ln);
  int rm, rn;
  helper(root->right, &rm, &rn);

  *m = max(lm, ln) + max(rm, rn);
  *n = root->val + lm + rm;
}
int rob(struct TreeNode *root) {
  int a, b;
  helper(root, &a, &b);
  return max(a, b);
}
