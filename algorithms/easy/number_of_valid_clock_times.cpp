class Solution {
public:
  int countTime(string time) {
    int ct = 1;
    if (time[0] == '?') {
      ct = time[1] == '?' ? 24 : (time[1] < '4' ? 3 : 2);
    } else if (time[1] == '?') {
      ct = time[0] == '2' ? 4 : 10;
    }

    if (time[3] == '?') ct *= 6;
    if (time[4] == '?') ct *= 10;
    
    return ct;
  }
};
