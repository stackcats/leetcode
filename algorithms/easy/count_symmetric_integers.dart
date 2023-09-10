bool isSymmetric(int n) {
  List<int> arr = [];
  while (n > 0) {
    arr.add(n % 10);
    n ~/= 10;
  }
  if (arr.length.isOdd) {
    return false;
  }

  int left = arr.sublist(0, arr.length ~/ 2).reduce((a, b) => a + b);
  int right = arr.sublist(arr.length ~/ 2).reduce((a, b) => a + b);
  return left == right;
}

class Solution {
  int countSymmetricIntegers(int low, int high) {
    int ct = 0;
    for (int i = low; i <= high; i++) {
      if (isSymmetric(i)) {
        ct++;
      }
    }
    return ct;
  }
}
