// https://leetcode.com/problems/add-two-numbers-ii/description/
// 使用栈实现翻转链表
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

typedef struct Node_ {
  void *data;
  struct Node_ *next;
} Node;

Node *node_init(void *data) {
  Node *node = (Node *)malloc(sizeof(Node));
  node->data = data;
  node->next = NULL;
  return node;
}

typedef struct {
  Node *base;
} Stack;

Stack *stack_init() {
  Stack *st = (Stack *)malloc(sizeof(Stack));
  st->base = NULL;
  return st;
}

#define stack_is_empty(st) ((st)->base == NULL)

void stack_free(Stack *st) {
  if (!st) {
    return;
  }

  while (st->base) {
    Node *node = st->base;
    st->base = node->next;
    free(node);
  }

  free(st);
}

Stack *push(Stack *st, void *data) {
  if (!st) {
    return NULL;
  }

  Node *node = node_init(data);
  node->next = st->base;
  st->base = node;

  return st;
}

void *pop(Stack *st) {
  if (stack_is_empty(st)) {
    return NULL;
  }

  Node *node = st->base;
  st->base = st->base->next;

  void *data = node->data;
  free(node);
  return data;
}

struct ListNode *cons(struct ListNode *l, int val) {
  struct ListNode *node = (struct ListNode *)malloc(sizeof(struct ListNode));
  node->val = val;
  node->next = l;
  return node;
}

struct ListNode *addTwoNumbers(struct ListNode *l1, struct ListNode *l2) {
  Stack *s1 = stack_init();
  while (l1) {
    push(s1, l1);
    l1 = l1->next;
  }

  Stack *s2 = stack_init();
  while (l2) {
    push(s2, l2);
    l2 = l2->next;
  }

  struct ListNode *res = NULL;
  struct ListNode *d1 = NULL;
  struct ListNode *d2 = NULL;
  int carry = 0;

  while (!stack_is_empty(s1) && !stack_is_empty(s2)) {
    d1 = pop(s1);
    d2 = pop(s2);
    int sum = d1->val + d2->val + carry;
    res = cons(res, sum % 10);
    carry = sum / 10;
  }

  while (!stack_is_empty(s1)) {
    d1 = pop(s1);
    int sum = d1->val + carry;
    res = cons(res, sum % 10);
    carry = sum / 10;
  }

  while (!stack_is_empty(s2)) {
    d2 = pop(s2);
    int sum = d2->val + carry;
    res = cons(res, sum % 10);
    carry = sum / 10;
  }

  if (carry > 0) {
    res = cons(res, carry);
  }

  stack_free(s1);
  stack_free(s2);
  return res;
}
