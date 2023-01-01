class Solution {
public:
    int countDigits(int num) {
      int ct{0};
      int n{num};
      while (n > 0) {
        if (num % (n % 10) == 0) {
          ct++;
        }
        n /= 10;
      }
      return ct;
    }
};
