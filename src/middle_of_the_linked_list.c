// https://leetcode.com/problems/middle-of-the-linked-list/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *middleNode(struct ListNode *head) {
  struct ListNode *p = head;
  struct ListNode *q = head;

  while (q && q->next) {
    q = q->next->next;
    p = p->next;
  }

  return p;
}
