class Solution {
  String maximumOddBinaryNumber(String s) {
    String l = '';
    String r = '';
    for (int i = 0; i < s.length; i++) {
      final c = s[i];
      if (c == '1') {
        l += c;
      } else {
        r += c;
      }
    }
    if (l.length == 1) {
      return r + l;
    }
    return l.substring(0, l.length - 1) + r + l.substring(l.length - 1);
  }
}
