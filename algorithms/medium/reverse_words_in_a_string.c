#include <stdlib.h>
#include <string.h>

typedef struct Vector {
  void **arr;
  size_t capacity;
  size_t size;
} Vector;

Vector* vector_init(size_t cap) {
  Vector* vec = malloc(sizeof(Vector));
  vec->capacity = cap;
  vec->size = 0;
  vec->arr = malloc(sizeof(void*) *cap);
  return vec;
}

void vector_resize(Vector *v, int cap) {
  void **arr = realloc(v->arr, sizeof(void*) * cap);
  if (arr) {
    v->arr = arr;
    v->capacity = cap;
  }
}

void vector_add(Vector *v, void *item) {
  if (v->capacity == v->size) {
    vector_resize(v, v->capacity * 2);
  }
  v->arr[v->size++] = item;
}

void vector_free(Vector *v) {
  if (v) {
    free(v->arr);
    free(v);
  }
}

char * reverse_join(Vector *vec, char* sep) {
  size_t len = 0;
  for (size_t i = 0; i < vec->size; i++) {
    len += strlen((char*)vec->arr[i]);    
  }

  len++;
  len += strlen(sep) * (vec->size - 1);

  char *s = (char*)malloc(len);
  memset(s, 0, len);
  for (int i = vec->size - 1; i >= 0; i--) {
    strcat(s, (char*)vec->arr[i]);
    if (i > 0) {
      strcat(s, sep);
    }
  }
  return s;
}

char * reverseWords(char * s){
  Vector *v = vector_init(1024);
  char *p = strtok(s, " ");
  while (p != NULL) {
    vector_add(v, p);
    p = strtok(NULL, " ");
  }
  char *t = reverse_join(v, " ");
  vector_free(v);
  return t;
}
