bool isPalindrome(char *s) {
  int i = 0;
  int j = strlen(s) - 1;
  while (i < j) {
    if (!isalnum(s[i])) {
      i++;
      continue;
    }

    if (!isalnum(s[j])) {
      j--;
      continue;
    }

    if (tolower(s[i]) != tolower(s[j])) {
      return false;
    }

    i++;
    j--;
  }

  return true;
}
