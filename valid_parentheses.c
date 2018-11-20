// https://leetcode.com/problems/valid-parentheses/description/

bool isValid(char *s) {
  int len = strlen(s);
  char *st = (char *)malloc(sizeof(char) * len);
  int top = 0;

  while (*s != '\0') {
    switch (*s) {
    case '(':
    case '[':
    case '{':
      st[top] = *s;
      top++;
      break;
    case ')':
      if (st[top - 1] == '(') {
        top--;
      } else {
        return false;
      }
      break;
    case ']':
      if (st[top - 1] == '[') {
        top--;
      } else {
        return false;
      }
      break;
    case '}':
      if (st[top - 1] == '{') {
        top--;
      } else {
        return false;
      }
      break;
    }
    s++;
  }

  free(st);

  return top == 0;
}
