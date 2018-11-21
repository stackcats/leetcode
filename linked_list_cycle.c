// https://leetcode.com/problems/linked-list-cycle/description/

// 使用快慢指针 快指针每次两步 慢指针每次一步
// 如果两个指针能相遇 则存在环
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
bool hasCycle(struct ListNode *head) {
  struct ListNode *slow = head;
  struct ListNode *fast = head;

  while (fast && fast->next) {
    fast = fast->next->next;
    slow = slow->next;
    if (slow == fast)
      return 1;
  }
  return 0;
}
