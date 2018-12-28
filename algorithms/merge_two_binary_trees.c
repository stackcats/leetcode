// https://leetcode.com/problems/merge-two-binary-trees/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
struct TreeNode *mergeTrees(struct TreeNode *t1, struct TreeNode *t2) {
  if (!t1 && !t2) {
    return NULL;
  }

  if (!t1) {
    return t2;
  }

  if (!t2) {
    return t1;
  }

  t1->val += t2->val;
  t1->left = mergeTrees(t1->left, t2->left);
  t1->right = mergeTrees(t1->right, t2->right);
  return t1;
}
