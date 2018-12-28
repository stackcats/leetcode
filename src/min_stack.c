// https://leetcode.com/problems/min-stack/description/

typedef struct STNode_ {
  int val;
  struct STNode_ *next;
} STNode;

STNode *node_init(int val) {
  STNode *node = (STNode *)malloc(sizeof(STNode));
  node->val = val;
  node->next = NULL;
  return node;
}

typedef struct {
  STNode *base;
  int size;
} Stack;

Stack *stack_init(int size) {
  Stack *st = (Stack *)malloc(sizeof(Stack));
  st->base = NULL;
  st->size = size;
  return st;
}

void stack_free(Stack *st) {
  assert(st);

  while (st->base) {
    STNode *node = st->base;
    st->base = st->base->next;
    free(node);
  }

  free(st);
}

void stack_push(Stack *st, int val) {
  assert(st);
  STNode *node = node_init(val);
  node->next = st->base;
  st->base = node;
}

int stack_pop(Stack *st) {
  assert(st && st->base);
  STNode *node = st->base;
  st->base = st->base->next;
  int val = node->val;
  free(node);
  return val;
}

int stack_top(Stack *st) {
  // assert(st && st->base);
  int val = st->base->val;
  return val;
}

#define stack_is_empty(st) ((st)->base == NULL)

typedef struct {
  Stack *st;
  Stack *min;
} MinStack;

/** initialize your data structure here. */
MinStack *minStackCreate(int maxSize) {
  // maxSize 没用上
  MinStack *ms = (MinStack *)malloc(sizeof(MinStack));
  ms->st = stack_init(maxSize);
  ms->min = stack_init(maxSize);
  return ms;
}

void minStackPush(MinStack *obj, int x) {
  assert(obj);
  stack_push(obj->st, x);
  if (stack_is_empty(obj->min) || x <= stack_top(obj->min)) { // 注意是<=
    stack_push(obj->min, x);
  }
}

void minStackPop(MinStack *obj) {
  assert(obj);
  int val = stack_pop(obj->st);
  if (!stack_is_empty(obj->min) && val == stack_top(obj->min)) {
    stack_pop(obj->min);
  }
}

int minStackTop(MinStack *obj) {
  assert(obj);
  return stack_top(obj->st);
}

int minStackGetMin(MinStack *obj) {
  assert(obj);
  return stack_top(obj->min);
}

void minStackFree(MinStack *obj) {
  if (!obj) {
    return;
  }

  stack_free(obj->st);
  stack_free(obj->min);
  free(obj);
}

/**
 * Your MinStack struct will be instantiated and called as such:
 * struct MinStack* obj = minStackCreate(maxSize);
 * minStackPush(obj, x);
 * minStackPop(obj);
 * int param_3 = minStackTop(obj);
 * int param_4 = minStackGetMin(obj);
 * minStackFree(obj);
 */
