// https://leetcode.com/problems/count-primes/description/

int countPrimes(int n) {
  if (n <= 2) {
    return 0;
  }

  int *arr = calloc(n, sizeof(int));
  int ct = 1;
  int end = ceil(sqrt(n));
  for (int i = 3; i < end; i += 2) {
    if (arr[i] == 0) {
      for (int k = i * i; k < n; k += 2 * i) {
        arr[k] = 1;
      }
    }
  }

  for (int i = 3; i < n; i += 2) {
    if (arr[i] == 0) {
      ct++;
    }
  }
  free(arr);
  return ct;
}
