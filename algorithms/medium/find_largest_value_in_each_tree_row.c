// https://leetcode.com/problems/find-largest-value-in-each-tree-row/description/

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
typedef struct ListNode_ {
  void *data;
  struct ListNode_ *next;
} ListNode;

typedef struct {
  ListNode *front;
  ListNode *rear;
  int size;
} Queue;

Queue *queue_init() {
  Queue *q = malloc(sizeof(Queue));
  q->front = NULL;
  q->rear = NULL;
  q->size = 0;
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
  q->size += 1;
}

void *deqeue(Queue *q) {
  ListNode *node = q->front;
  if (q->front == NULL) {
    q->rear = NULL;
  } else {
    q->front = q->front->next;
  }

  q->size -= 1;
  void *data = node->data;
  free(node);
  return data;
}

#define queue_is_empty(q) ((q)->front == NULL)

/**
 * Return an array of size *returnSize.
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *largestValues(struct TreeNode *root, int *returnSize) {
  if (!root) {
    *returnSize = 0;
    return NULL;
  }

  Queue *q1 = queue_init();
  Queue *q2 = queue_init();
  enqueue(q1, root);

  int *arr = calloc(1024, sizeof(int));
  *returnSize = 0;

  struct TreeNode *node;
  while (!queue_is_empty(q1)) {
    int max = INT_MIN;
    while (!queue_is_empty(q1)) {
      node = deqeue(q1);
      if (max < node->val) {
        max = node->val;
      }
      if (node->left) {
        enqueue(q2, node->left);
      }
      if (node->right) {
        enqueue(q2, node->right);
      }
    }

    arr[*returnSize] = max;
    *returnSize += 1;

    Queue *t = q1;
    q1 = q2;
    q2 = t;
  }

  free(q1);
  free(q2);
  return arr;
}
