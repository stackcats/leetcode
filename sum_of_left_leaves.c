// https://leetcode.com/problems/sum-of-left-leaves/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

typedef enum { left, right } Direction;

#define is_leaf(t) ((t)->left == NULL && (t)->right == NULL)

int sumOfLeftLeavesWithDirection(struct TreeNode *root, Direction d) {
  if (!root) {
    return 0;
  }

  int m = is_leaf(root) && d == left ? root->val : 0;
  int l = sumOfLeftLeavesWithDirection(root->left, left);
  int r = sumOfLeftLeavesWithDirection(root->right, right);

  return l + m + r;
}

int sumOfLeftLeaves(struct TreeNode *root) {
  if (!root) {
    return 0;
  }

  int l = sumOfLeftLeavesWithDirection(root->left, left);
  int r = sumOfLeftLeavesWithDirection(root->right, right);
  return l + r;
}
