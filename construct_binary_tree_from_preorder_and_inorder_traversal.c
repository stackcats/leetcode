// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

struct TreeNode *helper(int *preorder, int ps, int pe, int *inorder, int is,
                        int ie) {
  if (is > ie || ps > pe) {
    return NULL;
  }

  int r = preorder[ps]; // 根
  struct TreeNode *root = (struct TreeNode *)malloc(sizeof(struct TreeNode));
  root->val = r;

  int i = is;
  while (inorder[i] != r) {
    i++;
  }

  int leftLen = i - is; // 左子树的长度
  root->left = helper(preorder, ps + 1, ps + leftLen, inorder, is, i - 1);
  root->right = helper(preorder, ps + leftLen + 1, pe, inorder, i + 1, ie);

  return root;
}
struct TreeNode *buildTree(int *preorder, int preorderSize, int *inorder,
                           int inorderSize) {
  return helper(preorder, 0, preorderSize - 1, inorder, 0, inorderSize - 1);
}
