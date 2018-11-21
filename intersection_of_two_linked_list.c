// https://leetcode.com/problems/intersection-of-two-linked-lists/description/

// 指针a,b分别遍历一边listA,listB
// 由于遍历的步长相等 最终会在相交处或者'\0'相等

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *getIntersectionNode(struct ListNode *headA,
                                     struct ListNode *headB) {
  if (!headA || !headB)
    return NULL;

  struct ListNode *a = headA, *b = headB;

  while (a != b) {
    a = a ? a->next : headB;
    b = b ? b->next : headA;
  }

  return a;
}
