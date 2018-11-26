// https://leetcode.com/problems/copy-list-with-random-pointer/description/

/**
 * Definition for singly-linked list with a random pointer.
 * struct RandomListNode {
 *     int label;
 *     struct RandomListNode *next;
 *     struct RandomListNode *random;
 * };
 */

struct RandomListNode *node_init(int label) {
  struct RandomListNode *node =
      (struct RandomListNode *)malloc(sizeof(struct RandomListNode));
  node->label = label;
  node->random = NULL;
  node->next = NULL;
  return node;
}

struct RandomListNode *copyRandomList(struct RandomListNode *lst) {
  struct RandomListNode *iter = lst;

  // 1. 将copy的节点暂时保存在原节点的后面
  while (iter) { // O(n)
    struct RandomListNode *node = node_init(iter->label);
    node->next = iter->next;
    iter->next = node;
    iter = node->next;
  }

  // 因为多个random可能指向同一个节点 下面的2,3步不能合成一步
  // 过早的复原原链表会破坏random指针的对应关系

  // 2. 处理copy节点的random指针  iter->next为copy节点
  iter = lst;
  while (iter) { // O(n)
    if (iter->random) {
      iter->next->random = iter->random->next;
    }

    iter = iter->next->next;
  }

  // 3. 构建copy链表 复原原链表
  struct RandomListNode *head = NULL;
  struct RandomListNode *tail = NULL;
  iter = lst;
  while (iter) { // O(n)
    struct RandomListNode *node = iter->next;
    iter->next = node->next;
    iter = node->next;

    if (head == NULL) {
      head = node;
    } else {
      tail->next = node;
    }

    tail = node;
  }

  // O(n)
  return head;
}
