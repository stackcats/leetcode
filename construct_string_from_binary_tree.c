// https://leetcode.com/problems/construct-string-from-binary-tree/description/

// 前序遍历

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
char *helper(struct TreeNode *t) {
  char *s = NULL;
  if (!t) {
    return "";
  }

  char *left, *right;
  if (t->left && t->right) {
    left = helper(t->left);
    right = helper(t->right);
    s = (char *)malloc(strlen(left) + strlen(right) + 32);
    sprintf(s, "%d(%s)(%s)", t->val, left, right);
    free(left);
    free(right);
  } else if (!t->left && t->right) {
    right = helper(t->right);
    s = (char *)malloc(strlen(right) + 32);
    sprintf(s, "%d()(%s)", t->val, right);
    free(right);
  } else if (t->left && !t->right) {
    left = helper(t->left);
    s = (char *)malloc(strlen(left) + 32);
    sprintf(s, "%d(%s)", t->val, left);
    free(left);
  } else {
    s = (char *)malloc(16);
    sprintf(s, "%d", t->val);
  }

  return s;
}
char *tree2str(struct TreeNode *t) { return helper(t); }
