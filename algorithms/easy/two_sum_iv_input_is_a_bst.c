// https://leetcode.com/problems/two-sum-iv-input-is-a-bst/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
struct TreeNode *find(struct TreeNode *root, int k) {
  if (!root) {
    return NULL;
  }

  if (root->val == k) {
    return root;
  }

  if (root->val < k) {
    return find(root->right, k);
  }

  return find(root->left, k);
}

int helper(struct TreeNode *t, struct TreeNode *root, int k) {
  if (!t) {
    return 0;
  }

  int target = find(root, k - t->val);
  if (target && target != t) {
    return 1;
  }

  return helper(t->left, root, k) || helper(t->right, root, k);
}
bool findTarget(struct TreeNode *root, int k) { return helper(root, root, k); }
