class Solution {
public:
  int findNthDigit(int n) {
    long digit_num = 1;
    long ct = 9;
    while (digit_num * ct < n) {
      n -= digit_num * ct;
      digit_num++;
      ct *= 10;
    }

    int target = pow(10, (digit_num - 1)) + (n - 1) / digit_num;
    string s = to_string(target);
    return s[(n - 1) % digit_num] - '0';
  }
};
