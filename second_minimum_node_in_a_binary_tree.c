// https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
void helper(struct TreeNode *root, int *min, int *secondMin) {
  if (!root) {
    return;
  }

  if (root->val < *min) {
    *secondMin = *min;
    *min = root->val;
  } else if (root->val > *min && // 注意这里root->val不能等于*min
             root->val < *secondMin) {
    *secondMin = root->val;
  }

  helper(root->left, min, secondMin);
  helper(root->right, min, secondMin);
}

int findSecondMinimumValue(struct TreeNode *root) {
  int min = INT_MAX;
  int secondMin = INT_MAX;
  helper(root, &min, &secondMin);
  return INT_MAX != secondMin ? secondMin : -1;
}
