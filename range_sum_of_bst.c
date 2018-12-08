// https://leetcode.com/problems/range-sum-of-bst/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
int rangeSumBST(struct TreeNode *root, int L, int R) {
  if (!root) {
    return 0;
  }

  if (root->val < L) {
    return rangeSumBST(root->right, L, R);
  }

  if (root->val > R) {
    return rangeSumBST(root->left, L, R);
  }

  return root->val + rangeSumBST(root->right, L, R) +
    rangeSumBST(root->left, L, R);
}
