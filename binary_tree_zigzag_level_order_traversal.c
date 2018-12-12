// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/description/

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
#define queue_size(q) ((q)->size)

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
/**
 * Return an array of arrays of size *returnSize.
 * The sizes of the arrays are returned as *columnSizes array.
 * Note: Both returned array and *columnSizes array must be malloced, assume
 * caller calls free().
 */
int **zigzagLevelOrder(struct TreeNode *root, int **columnSizes,
                       int *returnSize) {
  if (!root) {
    *returnSize = 0;
    return NULL;
  }

  Queue *q1 = queue_init();
  Queue *q2 = queue_init();
  enqueue(q1, root);

  int **arr = calloc(1024, sizeof(int *));
  *columnSizes = calloc(1024, sizeof(int));
  *returnSize = 0;

  struct TreeNode *node;
  int isLeft = 1;
  int i, di;
  while (!queue_is_empty(q1)) {
    arr[*returnSize] = calloc(queue_size(q1), sizeof(int));
    (*columnSizes)[*returnSize] = queue_size(q1);

    if (isLeft) {
      i = 0;
      di = 1;
    } else {
      i = queue_size(q1) - 1;
      di = -1;
    }
    while (!queue_is_empty(q1)) {
      node = deqeue(q1);
      arr[*returnSize][i] = node->val;
      i += di;
      if (node->left) {
        enqueue(q2, node->left);
      }
      if (node->right) {
        enqueue(q2, node->right);
      }
    }

    isLeft = !isLeft;
    *returnSize += 1;

    Queue *t = q1;
    q1 = q2;
    q2 = t;
  }

  free(q1);
  free(q2);
  return arr;
}
