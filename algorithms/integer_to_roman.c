// https://leetcode.com/problems/integer-to-roman/description/

char *thousand[4] = {"", "M", "MM", "MMM"};
char *hundred[10] = {"",  "C",  "CC",  "CCC",  "CD",
                     "D", "DC", "DCC", "DCCC", "CM"};
char *ten[10] = {"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"};
char *digit[10] = {"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"};

char *intToRoman(int num) {
  char *s = (char *)malloc(sizeof(char) * 256);
  memset(s, 0, sizeof(char) * 256);

  strcat(s, thousand[num / 1000]);
  num %= 1000;
  strcat(s, hundred[num / 100]);
  num %= 100;
  strcat(s, ten[num / 10]);
  strcat(s, digit[num % 10]);

  return s;
}
