// https://leetcode.com/problems/ugly-number-ii/description/

#define min(a, b) ((a) < (b) ? (a) : (b))

int nthUglyNumber(int n) {
  int arr[2048];
  memset(arr, 0, sizeof(arr));

  arr[0] = 1;

  // i2,i3,i5分别记录乘2,3,5得到最小值的下标
  int i2 = 0;
  int i3 = 0;
  int i5 = 0;

  int n2, n3, n5;
  for (int i = 0; i < n; i++) {
    // 通过3个下标计算下一个最小的ugly number
    n2 = arr[i2] * 2;
    n3 = arr[i3] * 3;
    n5 = arr[i5] * 5;
    int m = min(n2, min(n3, n5));

    // 计算下一个最小值是由i2,i3,i5哪个值产生并更新i2,i3,i5指向新的最小值
    // 不能使用if else结构
    // 比如 6 = 2 * 3
    // 此时i2,i3都需要指向6
    if (n2 == m) {
      i2++;
    }

    if (n3 == m) {
      i3++;
    }

    if (n5 == m) {
      i5++;
    }

    arr[i + 1] = m;
  }

  return arr[n - 1];
}
