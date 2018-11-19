// https://leetcode.com/problems/regular-expression-matching/description/

// https://www.cs.princeton.edu/courses/archive/spr09/cos333/beautiful.html

bool isMatch(char *text, char *regexp) { return matchhere(regexp, text); }

int matchhere(char *regexp, char *text) {
  if (regexp[0] == '\0')
    return *text == '\0';
  if (regexp[1] == '*')
    return matchstar(regexp[0], regexp + 2, text);
  if (*text != '\0' && (regexp[0] == '.' || regexp[0] == *text))
    return matchhere(regexp + 1, text + 1);
  return 0;
}

int matchstar(int c, char *regexp, char *text) {
  do {
    if (matchhere(regexp, text))
      return 1;
  } while (*text != '\0' && (*text++ == c || c == '.'));
  return 0;
}
