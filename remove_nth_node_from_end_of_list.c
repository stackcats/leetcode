// https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *removeNthFromEnd(struct ListNode *head, int n) {
  struct ListNode *p = head;
  struct ListNode *q = head;
  struct ListNode *pre = NULL;

  while (q != NULL && n > 0) {
    q = q->next;
    n--;
  }

  while (q != NULL) {
    q = q->next;
    pre = p;
    p = p->next;
  }

  if (pre == NULL) {
    free(head);
    return p->next;
  }

  pre->next = p->next;
  free(p);
  return head;
}
