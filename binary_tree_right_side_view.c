// https://leetcode.com/problems/binary-tree-right-side-view/description/

typedef struct ListNode_ {
  void *data;
  struct ListNode_ *next;
} ListNode;

typedef struct {
  ListNode *front;
  ListNode *rear;
} Queue;

Queue *queue_init() {
  Queue *q = malloc(sizeof(Queue));
  q->front = NULL;
  q->rear = NULL;
  return q;
}

void queue_free(Queue *q) {
  while (q->front != NULL) {
    ListNode *node = q->front;
    q->front = q->front->next;
    free(node);
  }
  free(q);
}

void enqueue(Queue *q, void *data) {
  ListNode *node = malloc(sizeof(ListNode));
  node->data = data;
  node->next = NULL;

  if (q->front == NULL) {
    q->front = node;
  } else {
    q->rear->next = node;
  }

  q->rear = node;
}

void *deqeue(Queue *q) {
  ListNode *node = q->front;
  if (q->front == NULL) {
    q->rear = NULL;
  } else {
    q->front = q->front->next;
  }

  void *data = node->data;
  free(node);
  return data;
}

#define queue_is_empty(q) ((q)->front == NULL)
#define queue_last(q) ((q)->rear->data);

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
/**
 * Return an array of size *returnSize.
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *rightSideView(struct TreeNode *root, int *returnSize) {
  if (!root) {
    *returnSize = 0;
    return NULL;
  }

  Queue *q1 = queue_init();
  Queue *q2 = queue_init();
  enqueue(q1, root);

  int *arr = calloc(1024, sizeof(int));
  *returnSize = 0;

  struct TreeNode *rightMost, node;

  while (!queue_is_empty(q1)) {
    rightMost = queue_last(q1);

    arr[*returnSize] = rightMost->val;
    *returnSize += 1;

    while (!queue_is_empty(q1)) {
      struct TreeNode *node = deqeue(q1);
      if (node->left) {
        enqueue(q2, node->left);
      }
      if (node->right) {
        enqueue(q2, node->right);
      }
    }

    Queue *t = q1;
    q1 = q2;
    q2 = q1;
  }

  free(q1);
  free(q2);
  return arr;
}
