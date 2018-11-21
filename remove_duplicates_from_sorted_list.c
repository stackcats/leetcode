// https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *deleteDuplicates(struct ListNode *head) {
  struct ListNode *iter = head, *next, *tmp;
  while (iter) {
    next = iter->next;
    while (next && next->val == iter->val) {
      tmp = next;
      next = next->next;
      free(tmp);
    }
    iter->next = next;
    iter = next;
  }
  return head;
}
