#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdbool.h>

typedef struct {
  int n;
  int curr;
  pthread_mutex_t m;
  pthread_cond_t cond;
} FizzBuzz;

FizzBuzz* fizzBuzzCreate(int n) {
  FizzBuzz* obj = (FizzBuzz*) malloc(sizeof(FizzBuzz));
  obj->n = n;
  obj->curr = 1;
  pthread_mutex_init(&obj->m, NULL);
  pthread_cond_init(&obj->cond, NULL);
  return obj;
}

// printFizz() outputs "fizz".
void fizz(FizzBuzz* obj) {
  while (true) {
    pthread_mutex_lock(&obj->m);

    while (obj->curr <= obj->n && (obj->curr % 3 != 0 || obj->curr % 5 == 0)) {
      pthread_cond_wait(&obj->cond, &obj->m);
    }
    if (obj->curr > obj->n) {
      pthread_mutex_unlock(&obj->m);
      return;
    }
    printFizz();
    obj->curr += 1;
    pthread_cond_broadcast(&obj->cond);
    pthread_mutex_unlock(&obj->m);
  }
}

// printBuzz() outputs "buzz".
void buzz(FizzBuzz* obj) {
  while (true) {
    pthread_mutex_lock(&obj->m);

    while (obj->curr <= obj->n && (obj->curr % 5 != 0 || obj->curr % 3 == 0)) {
      pthread_cond_wait(&obj->cond, &obj->m);
    }
    if (obj->curr > obj->n) {
      pthread_mutex_unlock(&obj->m);
      return;
    }
    printBuzz();
    obj->curr += 1;
    pthread_cond_broadcast(&obj->cond);
    pthread_mutex_unlock(&obj->m);
  }
}

// printFizzBuzz() outputs "fizzbuzz".
void fizzbuzz(FizzBuzz* obj) {
  while (true) {
    pthread_mutex_lock(&obj->m);

    while (obj->curr <= obj->n && obj->curr % 15 != 0) {
      pthread_cond_wait(&obj->cond, &obj->m);
    }
    if (obj->curr > obj->n) {
      pthread_mutex_unlock(&obj->m);
      return;
    }
    printFizzBuzz();
    obj->curr += 1;
    pthread_cond_broadcast(&obj->cond);
    pthread_mutex_unlock(&obj->m);
  }
}

// You may call global function `void printNumber(int x)`
// to output "x", where x is an integer.
void number(FizzBuzz* obj) {
  while (true) {
    pthread_mutex_lock(&obj->m);

    while (obj->curr <= obj->n && (obj->curr % 3 == 0 || obj->curr % 5 == 0)) {
      pthread_cond_wait(&obj->cond, &obj->m);
    }
    if (obj->curr > obj->n) {
      pthread_mutex_unlock(&obj->m);
      return;
    }
    printNumber(obj->curr);
    obj->curr += 1;
    pthread_cond_broadcast(&obj->cond);
    pthread_mutex_unlock(&obj->m);
  }
}

void fizzBuzzFree(FizzBuzz* obj) {
  pthread_mutex_destroy(&obj->m);
  pthread_cond_destroy(&obj->cond);
  free(obj);
}
