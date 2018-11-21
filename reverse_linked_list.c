// https://leetcode.com/problems/reverse-linked-list/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *reverseList(struct ListNode *head) {
  struct ListNode *rev = NULL;

  while (head) {
    struct ListNode *tmp = head;
    head = head->next;
    tmp->next = rev;
    rev = tmp;
  }
  return rev;
}
