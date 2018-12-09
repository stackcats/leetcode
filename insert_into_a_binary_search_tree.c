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
  if (!root) {
    TreeNode *t = malloc(sizeof(TreeNode));
    t->val = val;
    t->right = t->left = NULL;
    return t;
  }

  if (root->val > val) {
    root->left = insertIntoBST(root->left, val);
  } else {
    root->right = insertIntoBST(root->right, val);
  }
  return root;
}
