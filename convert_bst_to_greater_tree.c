// https://leetcode.com/problems/convert-bst-to-greater-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

struct TreeNode *helper(struct TreeNode *root, int *parnet) {
  if (!root) {
    return NULL;
  }

  root->right = helper(root->right, parnet);
  *parnet += root->val;
  root->val = *parnet;
  root->left = helper(root->left, parnet);

  return root;
}

struct TreeNode *convertBST(struct TreeNode *root) {
  int p = 0;
  return helper(root, &p);
}
