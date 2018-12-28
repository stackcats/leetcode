// https://leetcode.com/problems/insert-into-a-binary-search-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
typedef struct TreeNode TreeNode;
struct TreeNode *insertIntoBST(struct TreeNode *root, int val) {
  TreeNode *parnet = NULL;
  TreeNode *iter = root;
  int isLeft = 1;
  while (iter) {
    if (iter->val > val) {
      parnet = iter;
      iter = iter->left;
      isLeft = 1;
    } else {
      parnet = iter;
      iter = iter->right;
      isLeft = 0;
    }
  }

  TreeNode *t = malloc(sizeof(TreeNode));
  t->val = val;
  t->right = t->left = NULL;
  if (parnet == NULL) {
    return t;
  }

  if (isLeft) {
    parnet->left = t;
  } else {
    parnet->right = t;
  }

  return root;
}
