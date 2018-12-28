// https://leetcode.com/problems/median-of-two-sorted-arrays/description/

#define max(a, b) ((a) > (b) ? (a) : (b))
#define min(a, b) ((a) < (b) ? (a) : (b))

double findMedianSortedArrays(int *a1, int n, int *a2, int m) {
  if (n > m) {
    return findMedianSortedArrays(a2, m, a1, n);
  }

  // 采用manacher算法的思想填充数组 使数组个数为2n+1恒为奇数
  // 中位数位置为(2n+1)/2 = n
  int lo = 0;
  int hi = 2 * n; // 填充后数组的最右边界

  int c1, c2; // 记录填充后数组的割点

  int l1, l2; // 割点左边的值 映射关系 l=(c-1)/2
  int r1, r2; // 割点右边的值 映射关系 r=c/2

  while (lo <= hi) {
    c1 = lo + (hi - lo) / 2; // (lo + hi) / 2 可能溢出
    c2 = m + n - c1;

    // 边界处理
    // ci == 0 说明ai的值都比中位数大
    // 并且li不存在 所以将li赋值为INT_MIN 使满足li < ri
    l1 = (c1 == 0) ? INT_MIN : a1[(c1 - 1) / 2];
    l2 = (c2 == 0) ? INT_MIN : a2[(c2 - 1) / 2];

    // ci == 2 * n 说明ai的值都比中位数小
    // 并且ri不存在 所以将ri赋值为INT_MAX 使满足li < ri
    r1 = (c1 == 2 * n) ? INT_MAX : a1[c1 / 2];
    r2 = (c2 == 2 * m) ? INT_MAX : a2[c2 / 2];

    if (l1 > r2) {
      // 中位数在c1左边 更新上界
      hi = c1 - 1;
    } else if (l2 > r1) {
      // 中位数在c1右边 更新下界
      lo = c1 + 1;
    } else {
      // l1 <= r2 && l2 <= r2时 找到解 退出循环
      break;
    }
  }

  return (max(l1, l2) + min(r1, r2)) / 2.0;
}
