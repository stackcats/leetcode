/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */


struct ListNode* mergeInBetween(struct ListNode* list1, int a, int b, struct ListNode* list2){
  struct ListNode* start = list1;
  while (start->next->val != a) {
    start = start->next;
  }
  struct ListNode* end = start->next;
  while (end->val != b) {
    end = end->next;
  }
  start->next = list2;
  while (start->next != NULL) {
    start = start->next;
  }
  start->next = end->next;
  return list1;
}
