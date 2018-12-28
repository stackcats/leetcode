// https://leetcode.com/problems/count-complete-tree-nodes/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
int countNodes(struct TreeNode *root) {
  if (!root) {
    return 0;
  }

  struct TreeNode *left = root->left;
  struct TreeNode *right = root->right;
  int ct = 1;
  while (left && right) { // 判断是否是满二叉树
    ct++;
    left = left->left;
    right = right->right;
  }

  if (!left && !right) { // 满二叉树直接返回个数
    return (1 << ct) - 1;
  }

  return countNodes(root->left) + countNodes(root->right) + 1;
}
