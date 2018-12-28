// https://leetcode.com/problems/symmetric-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

struct TreeNode *invert(struct TreeNode *root) {
  if (!root) {
    return NULL;
  }

  struct TreeNode *t = root->left;
  root->left = invert(root->right);
  root->right = invert(t);
  return root;
}

bool isEqual(struct TreeNode *t1, struct TreeNode *t2) {
  if (!t1 && !t2) {
    return 1;
  }

  if (!t1 || !t2) {
    return 0;
  }

  if (t1->val != t2->val) {
    return 0;
  }

  return isEqual(t1->left, t2->left) && isEqual(t1->right, t2->right);
}

bool isSymmetric(struct TreeNode *root) {
  return !root || isEqual(invert(root->left), root->right);
}
