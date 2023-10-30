#include <iostream>
#include <map>
#include <string>
#include <vector>

#define MIN(a, b) (a < b) ? (a) : (b)

using std::map;
using std::string;
using std::vector;

typedef map<char, int> edges;
typedef vector<edges> trie;

void build_mini_trie(string pattern, trie &t) {
  int i = 0;
  for (auto c : pattern) {
    edges e;
    e[c] = ++i;
    t.push_back(e);
  }
}

void merge_trie(string pattern, trie &t) {
  int i = 0;
  int lp = pattern.size();
  int lt = t.size();
  int l = MIN(lp, lt);
  bool f = false;

  int idx = 0;
  for (i = 0; i < lt; i++) {
    idx += t[i].size();
  }
  idx += 1;

  for (i = 0; i < l; i++) {
    f = false;
    for (const auto &c : t[i]) {
      if (c.first != pattern[i]) continue;
      f = true;
      break;
    }
    if (!f) break;
  }

  if (i >= lp) return;

  for (int j = 0; j < idx + lp - lt; j++) {
    edges e;
    t.push_back(e);
  }

  t[i][pattern[i]] = idx;
  i += 1;

  for (; i < lp; i++) {
    int ii = idx++;
    t[ii][pattern[i]] = idx;
  }
}

trie build_trie(vector<string> &patterns) {
  trie t;

  for (auto pattern : patterns) {
    if (t.empty()) {
      build_mini_trie(pattern, t);
      continue;
    }
    merge_trie(pattern, t);
  }

  return t;
}

int main() {
  size_t n;
  std::cin >> n;
  vector<string> patterns;
  for (size_t i = 0; i < n; i++) {
    string s;
    std::cin >> s;
    patterns.push_back(s);
  }

  trie t = build_trie(patterns);
  for (size_t i = 0; i < t.size(); ++i) {
    for (const auto &j : t[i]) {
      std::cout << i << "->" << j.second << ":" << j.first << "\n";
    }
  }

  return 0;
}