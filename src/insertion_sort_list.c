// https://leetcode.com/problems/insertion-sort-list/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode *insert(struct ListNode *head, struct ListNode *node) {
  struct ListNode *pre = NULL;
  struct ListNode *curr = head;
  while (curr && curr->val < node->val) {
    pre = curr;
    curr = curr->next;
  }

  if (pre == NULL) {
    head = node;
  } else {
    pre->next = node;
  }

  node->next = curr;

  return head;
}

struct ListNode *insertionSortList(struct ListNode *head) {
  if (head == NULL) {
    return NULL;
  }

  struct ListNode *rest = head->next;
  if (rest == NULL) {
    return head;
  }

  return insert(insertionSortList(rest), head);
}
