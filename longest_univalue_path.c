// https://leetcode.com/problems/longest-univalue-path/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

inline int max(int a, int b) { return a < b ? b : a; }

int helper(struct TreeNode *root, int *m) {
  if (!root) {
    return 0;
  }

  int left = helper(root->left, m);
  int right = helper(root->right, m);

  if (root->left && root->right) {
    if (root->left->val == root->val && root->right->val == root->val) {
      *m = max(*m, left + right + 1);
      return max(left, right) + 1;
    }

    if (root->left->val == root->val) {
      *m = max(*m, left + 1);
      return left + 1;
    }

    if (root->right->val == root->val) {
      *m = max(*m, right + 1);
      return right + 1;
    }
  }

  if (root->left && root->left->val == root->val) {
    *m = max(*m, left + 1);
    return left + 1;
  }

  if (root->right && root->right->val == root->val) {
    *m = max(*m, right + 1);
    return right + 1;
  }

  *m = max(*m, 1);
  return 1;
}

int longestUnivaluePath(struct TreeNode *root) {
  int a = 1;
  helper(root, &a);
  return a - 1; // helper计算相同元素的个数，路径需要减一
}
