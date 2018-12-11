// https://leetcode.com/problems/largest-rectangle-in-histogram/description/

inline max(int a, int b) { return a < b ? b : a; }

typedef struct {
  int *arr;
  int cap;
  int top;
} Stack;

Stack *stack_init() {
  Stack *st = malloc(sizeof(Stack));
  st->cap = 16;
  st->top = -1;
  st->arr = calloc(st->cap, sizeof(int));
  return st;
}

void stack_free(Stack *st) {
  free(st->arr);
  free(st);
}

void stack_push(Stack *st, int val) {
  if (st->top + 1 >= st->cap) {
    st->cap *= 2;
    st->arr = realloc(st->arr, st->cap * sizeof(int));
  }

  st->top += 1;
  st->arr[st->top] = val;
}

int stack_pop(Stack *st) {
  int val = st->arr[st->top];
  st->top -= 1;
  return val;
}

#define stack_is_empty(st) ((st)->top < 0)
#define stack_peek(st) ((st)->arr[(st)->top])

int largestRectangleArea(int *heights, int heightsSize) {
  Stack *st = stack_init();
  stack_push(st, -1); // heights[]均为非负数 -1作为边界方便处理

  int maxArea = 0;

  for (int i = 0; i < heightsSize; i++) {
    while (heights[i] < heights[stack_peek(st)]) {
      int h = heights[stack_pop(st)];

      // i-1为右边界下标  stack_peek为左边界下标
      int w = i - 1 - stack_peek(st);
      maxArea = max(h * w, maxArea);
    }
    stack_push(st, i);
  }

  while (!stack_is_empty(st)) {
    int h = heights[stack_pop(st)];
    int w = heightsSize - 1 - stack_peek(st);
    maxArea = max(h * w, maxArea);
  }

  stack_free(st);

  return maxArea;
}
