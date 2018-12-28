// https://leetcode.com/problems/climbing-stairs/description/

int climbStairs(int n) {
  int *arr = calloc(n + 1, sizeof(int));
  arr[0] = 1;
  arr[1] = 1;
  for (int i = 2; i <= n; i++) {
    arr[i] = arr[i - 1] + arr[i - 2];
  }
  int ans = arr[n];
  free(arr);
  return ans;
}
