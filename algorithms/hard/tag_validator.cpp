class Solution {
public:
  size_t curr = 0;
  string code;
  bool isValid(string code) {
    this->code = code;
    return parseContent();
  }
  bool parseContent() {
    stack<string> st;
    bool started = false;
    bool closed = false;
    if (code[curr] != '<') {
      return false;
    }
    while (curr < code.size()) {
      try {
        if (closed && st.empty()) {
          return false;
        }
        closed = false;
        if (code.substr(curr, 9) == "<![CDATA[") {
          if (!started) {
            return false;
          }
          curr += 9;
          parseCDATAContent();
        } else if (code.substr(curr, 2) == "</") {
          if (!started) {
            return false;
          }
          curr += 2;
          string tag_name = parseTagName();
          if (tag_name != st.top()) {
            return false;
          }
          st.pop();
          closed = true;
        } else if (code[curr] == '<') {
          started = true;
          curr++;
          string tag_name = parseTagName();
          st.push(tag_name);
        } else {
          curr++;
        }
      } catch (...) {
        return false;
      }
    }
    return closed && st.empty();
  }

  string parseTagName() {
    string name;
    while (curr < code.size() && code[curr] != '>') {
      if (code[curr] < 'A' || code[curr] > 'Z') {
        throw invalid_argument("invalid TAG_NAME");
      }
      name.push_back(code[curr]);
      curr++;
    }
    if (curr == code.size()) {
      throw invalid_argument("invalid TAG_NAME");
    }
    if (name.size() < 1 || name.size() > 9) {
      throw invalid_argument("invalid TAG_NAME");
    }
    curr++;
    return name;
  }

  void parseCDATAContent() {
    while (curr < code.size()) {
      if (code.substr(curr, 3) == "]]>") {
        curr += 3;
        return;
      }
      curr++;
    }
    throw invalid_argument("unbalanced cdata");
  }
};
