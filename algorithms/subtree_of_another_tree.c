// https://leetcode.com/problems/subtree-of-another-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
bool isSame(struct TreeNode *s, struct TreeNode *t) {
  if (!s && !t) {
    return true;
  } else if (!s || !t) {
    return false;
  }

  if (s->val != t->val) {
    return false;
  }

  return isSame(s->left, t->left) && isSame(s->right, t->right);
}
bool isSubtree(struct TreeNode *s, struct TreeNode *t) {
  if (!s && !t) {
    return true;
  } else if (!s || !t) {
    return false;
  }

  return isSame(s, t) || isSubtree(s->left, t) || isSubtree(s->right, t);
}
