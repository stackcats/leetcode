// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

struct TreeNode *tree_node_init(int val) {
  struct TreeNode *t = (struct TreeNode *)malloc(sizeof(struct TreeNode));
  t->val = val;
  t->left = NULL;
  t->right = NULL;
  return t;
}

struct TreeNode *sortedListToBST(struct ListNode *head) {
  if (head == NULL) {
    return NULL;
  }

  // 使用快慢指针寻找链表的中点
  struct ListNode *pre = NULL;
  struct ListNode *slow = head;
  struct ListNode *fast = head;
  while (fast && fast->next) {
    fast = fast->next->next;
    pre = slow;
    slow = slow->next;
  }

  struct TreeNode *t = tree_node_init(slow->val);

  if (pre != NULL) {
    pre->next = NULL;
    t->left = sortedListToBST(head);
  }

  t->right = sortedListToBST(slow->next);

  return t;
}
