// https://leetcode.com/problems/palindrome-linked-list/description/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
bool isPalindrome(struct ListNode *lst) {
  // 快慢指针将lst分为两部分
  struct ListNode *pre = NULL;
  struct ListNode *slow = lst;
  struct ListNode *fast = lst;

  while (fast && fast->next) {
    fast = fast->next->next;
    pre = slow;
    slow = slow->next;
  }

  if (pre == NULL) { // lst长度为0或1
    return true;
  }

  pre->next = NULL;

  // 翻转第2部分链表
  struct ListNode *head = NULL;

  while (slow) {
    struct ListNode *node = slow;
    slow = slow->next;

    if (head == NULL) {
      head = node;
    } else {
      node->next = head;
      head = node;
    }
  }

  while (lst && head) {
    if (lst->val != head->val) {
      return false;
    }

    lst = lst->next;
    head = head->next;
  }

  // 第1部分比第2部分长
  if (lst) {
    return false;
  }

  // 第2部分比第1部分多一个节点 (奇数个节点的回文链表)
  if (head && head->next) {
    return true;
  }

  // 第2部分比第1部分长
  if (head) {
    return false;
  }

  return true; // 偶数个节点的回文链表
}
