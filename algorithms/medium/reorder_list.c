// https://leetcode.com/problems/reorder-list/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
void reorderList(struct ListNode *head) {
  if (!head) {
    return;
  }

  // 快慢指针分割链表
  struct ListNode *slow = head;
  struct ListNode *fast = head;
  while (fast && fast->next) {
    fast = fast->next->next;
    slow = slow->next;
  }

  struct ListNode *q = slow->next;
  slow->next = NULL;

  // 翻转后半部分链表
  struct ListNode *new = NULL;
  while (q) {
    struct ListNode *t = q;
    q = q->next;
    t->next = new;
    new = t;
  }

  // 合并两部分链表
  struct ListNode *p = head;
  q = new;
  while (p && q) {
    struct ListNode *x = p->next;
    struct ListNode *y = q->next;
    p->next = q;
    q->next = x;
    p = x;
    q = y;
  }

  return head;
}
