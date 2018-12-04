// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

struct TreeNode *helper(int *inorder, int is, int ie, int *postorder, int ps,
                        int pe) {
  if (is > ie || ps > pe) {
    return NULL;
  }

  int r = postorder[pe]; // 根
  struct TreeNode *root = (struct TreeNode *)malloc(sizeof(struct TreeNode));
  root->val = r;

  int i = is;
  while (inorder[i] != r) {
    i++;
  }

  int leftLen = i - is; // 左子树的长度
  root->left = helper(inorder, is, i - 1, postorder, ps, ps + leftLen - 1);
  root->right = helper(inorder, i + 1, ie, postorder, ps + leftLen, pe - 1);

  return root;
}

struct TreeNode *buildTree(int *inorder, int inorderSize, int *postorder,
                           int postorderSize) {
  return helper(inorder, 0, inorderSize - 1, postorder, 0, postorderSize - 1);
}
