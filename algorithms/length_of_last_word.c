int lengthOfLastWord(char *s) {
  int len = strlen(s);
  int i = len - 1;
  while (s[i] == ' ') {
    i--;
  }

  if (i < 0) {
    return 0;
  }

  int end = i;
  for (i = end; i >= 0 && s[i] != ' '; i--)
    ;

  if (i < 0) {
    return s[0] != ' ' ? end + 1 : 0;
  }

  return end - i;
}
