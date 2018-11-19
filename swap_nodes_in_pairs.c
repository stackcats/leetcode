/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *swapPairs(struct ListNode *head) {
  if (head == NULL)
    return head;

  struct ListNode *second = head->next;
  if (second == NULL)
    return head;

  struct ListNode *rest = swapPairs(second->next);
  head->next = rest;
  second->next = head;
  return second;
}
