// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *deleteDuplicates(struct ListNode *lst) {
  struct ListNode *iter = lst;

  struct ListNode *head = NULL;
  struct ListNode *tail = NULL;

  while (iter) {
    struct ListNode *next = iter->next;
    while (next && next->val == iter->val) {
      next = next->next;
    }

    if (iter->next == next) {
      if (head == NULL) {
        head = iter;
        tail = iter;
      } else {
        tail->next = iter;
        tail = iter;
      }
      tail->next = NULL;
    }

    iter = next;
  }

  return head;
}
