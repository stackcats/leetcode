// https://leetcode.com/problems/redundant-connection/description/

// 并查集
typedef struct {
  int *id;
  int *sz;
} UF;

UF *uf_init(int N) {
  UF *u = malloc(sizeof(UF));
  u->id = calloc(N, sizeof(int));
  u->sz = calloc(N, sizeof(int));
  for (int i = 0; i < N; i++) {
    u->id[i] = i;
    u->sz[i] = 1;
  }

  return u;
}

void uf_free(UF *u) {
  free(u->id);
  free(u->sz);
  free(u);
}

int uf_root(UF *u, int i) {
  while (u->id[i] != i) {
    i = u->id[i];
  }
  return i;
}

int uf_connected(UF *u, int p, int q) { return uf_root(u, p) == uf_root(u, q); }

void uf_union(UF *u, int p, int q) {
  int r1 = uf_root(u, p);
  int r2 = uf_root(u, q);
  if (u->sz[r1] < u->sz[r2]) {
    u->id[r1] = r2;
    u->sz[r2] += u->sz[r1];
  } else {
    u->id[r2] = r1;
    u->sz[r1] += u->sz[r2];
  }
}

/**
 * Return an array of size *returnSize.
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *findRedundantConnection(int **edges, int edgesRowSize, int edgesColSize,
                             int *returnSize) {
  UF *u = uf_init(2048);
  int *arr = calloc(2, sizeof(int));
  *returnSize = 2;
  for (int i = 0; i < edgesRowSize; i++) {
    int *edge = edges[i];
    if (!uf_connected(u, edge[0], edge[1])) {
      uf_union(u, edge[0], edge[1]);
    } else {
      arr[0] = edge[0];
      arr[1] = edge[1];
    }
  }

  free(u);
  return arr;
}
