class Solution {
public:
  int dateToDays(string s) {
    vector<int> days{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    int m = 0;
    int d = 0;
    sscanf(s.c_str(), "%d-%d", &m, &d);
    int ds = accumulate(days.begin(), days.begin() + m - 1, 0);
    return ds + d;
  }
  int countDaysTogether(string arriveAlice, string leaveAlice, string arriveBob, string leaveBob) {
    int alice_start = dateToDays(arriveAlice);
    int alice_end = dateToDays(leaveAlice);
    int bob_start = dateToDays(arriveBob);
    int bob_end = dateToDays(leaveBob);
    return max(0, min(alice_end, bob_end) - max(alice_start, bob_start) + 1);
  }
};
