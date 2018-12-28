// https://leetcode.com/problems/remove-linked-list-elements/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *removeElements(struct ListNode *head, int val) {
  struct ListNode *pre = NULL;
  struct ListNode *iter = head;

  struct ListNode *tmp = NULL;
  while (iter) {
    if (iter->val == val) {
      tmp = iter;
      if (pre == NULL) {
        head = iter->next;
      } else {
        pre->next = iter->next;
      }
      free(tmp);
    } else {
      pre = iter;
    }
    iter = iter->next;
  }

  return head;
}
