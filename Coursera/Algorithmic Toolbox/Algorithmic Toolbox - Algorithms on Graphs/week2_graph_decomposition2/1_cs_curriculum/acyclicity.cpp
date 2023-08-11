#include <iostream>
#include <vector>
#include <stack>
#include <set>
#include <algorithm>

using std::sort;
using std::set;
using std::stack;
using std::vector;
using std::pair;

typedef pair<int, int> pii;

int dfs(vector<vector<int>> &adj, int target, set<int> &visited) {
  stack<int> st = stack<int>();
  st.push(target);

  while (!st.empty()) {
    int n = st.top();
    st.pop();
    if (n == target && visited.find(n) != visited.end()) return 1;
    visited.insert(n);
    for (auto c : adj[n]) {
      if (c != target && visited.find(c) != visited.end()) continue;
      st.push(c);
    }
  }
  return 0;
}

int acyclic(vector<vector<int> > &adj) {
  set<int> visited = set<int>();

  for (int i = 0; i < adj.size(); i++) {
    if (visited.find(i) != visited.end()) continue;
    if (dfs(adj, i, visited)) return 1;
  }

  return 0;
}

int main() {
  size_t n, m;
  std::cin >> n >> m;
  vector<vector<int> > adj(n, vector<int>());
  for (size_t i = 0; i < m; i++) {
    int x, y;
    std::cin >> x >> y;
    adj[x - 1].push_back(y - 1);
  }
  std::cout << acyclic(adj);
}
