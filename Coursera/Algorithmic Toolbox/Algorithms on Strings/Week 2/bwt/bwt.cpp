#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using std::cin;
using std::cout;
using std::endl;
using std::pair;
using std::string;
using std::vector;

typedef pair<int, int> pci;

string BWT(const string& text) {
  string result = "";
  vector<pci> l;
  int ts = text.size();
  for (int i = 1; i <= ts; i++) {
    l.push_back(pci(i % ts, ts - i));
  }

  sort(l.begin(), l.end(), [text, ts](pci a, pci b) {
    char ca = text[a.second];
    char cb = text[b.second];

    if (ca != cb) return ca < cb;

    int ia = (a.second + 1) % ts;
    int ib = (b.second + 1) % ts;

    ca = text[ia];
    cb = text[ib];

    while (ca == cb) {
      ia = (ia + 1) % ts;
      ib = (ib + 1) % ts;
      ca = text[ia];
      cb = text[ib];
    }
    return ca < cb;
  });

  for (auto p : l) {
    result += text[ts - (p.first % ts) - 1];
  }

  return result;
}

int main() {
  string text;
  cin >> text;
  cout << BWT(text) << endl;
  return 0;
}