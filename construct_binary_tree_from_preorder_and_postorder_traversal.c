// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

struct TreeNode *helper(int *pre, int ps, int pe, int *post, int psts,
                        int pste) {
  if (ps > pe || psts > pste) {
    return NULL;
  }

  int r = pre[ps]; // 根
  struct TreeNode *root = (struct TreeNode *)malloc(sizeof(struct TreeNode));
  root->val = r;
  root->left = NULL;
  root->right = NULL;

  if (ps + 1 > pe) {
    return root;
  }

  int lr = pre[ps + 1];
  int i = psts;
  while (post[i] != lr) {
    i++;
  }

  int leftLen = i - psts + 1; // 左子树的长度
  root->left = helper(pre, ps + 1, ps + leftLen, post, psts, i);
  root->right = helper(pre, ps + leftLen + 1, pe, post, i + 1, pste - 1);

  return root;
}
struct TreeNode *constructFromPrePost(int *pre, int preSize, int *post,
                                      int postSize) {
  return helper(pre, 0, preSize - 1, post, 0, postSize - 1);
}
