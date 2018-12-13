// https://leetcode.com/problems/diameter-of-binary-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

inline max(int a, int b) { return a < b ? b : a; }

int helper(struct TreeNode *root, int *m) {
  if (!root) {
    return 0;
  }

  int left = helper(root->left, m);
  int right = helper(root->right, m);
  if (*m < left + right) {
    // 题目要求计算的是路径的个数 而不是节点的个数
    // 所以这里不是left+right+1
    *m = left + right;
  }

  return max(left, right) + 1;
}

int diameterOfBinaryTree(struct TreeNode *root) {
  int m = 0;
  helper(root, &m);
  return m;
}
