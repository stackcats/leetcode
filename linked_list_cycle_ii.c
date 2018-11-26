// https://leetcode.com/problems/linked-list-cycle-ii/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *detectCycle(struct ListNode *head) {
  struct ListNode *slow = head;
  struct ListNode *fast = head;

  while (fast && fast->next) {
    fast = fast->next->next;
    slow = slow->next;
    if (slow == fast) {
      while (head != slow) {
        head = head->next;
        slow = slow->next;
      }
      return head;
    }
  }

  return NULL;
}
