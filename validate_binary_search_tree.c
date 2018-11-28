// https://leetcode.com/problems/validate-binary-search-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

int helper(struct TreeNode *root, int *min, int *max) {
  if (!root) {
    return 1;
  }

  if (min && root->val <= *min) {
    return 0;
  }

  if (max && root->val >= *max) {
    return 0;
  }

  return helper(root->left, min, &root->val) &&
    helper(root->right, &root->val, max);
}

bool isValidBST(struct TreeNode *root) {
  int *min = NULL;
  int *max = NULL;
  return helper(root, min, max);
}
