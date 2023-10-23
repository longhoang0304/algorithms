#include <iostream>
#include <vector>
#include <set>
#include <stack>

using std::stack;
using std::set;
using std::vector;
using std::pair;


int reach(vector<vector<int> > &adj, int x, int y) {
  set<int> visited;
  stack<int> st;
  st.push(x);
  while (!st.empty()) {
    int node = st.top();
    st.pop();
    if (visited.find(node) != visited.end()) continue;
    visited.insert(node);
    for (int i = 0; i < adj[node].size(); i++) {
      int n = adj[node][i];
      if (visited.find(n) != visited.end()) continue;
      if (n ==  y) return 1;
      st.push(n);
    }
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
    adj[y - 1].push_back(x - 1);
  }
  int x, y;
  std::cin >> x >> y;
  std::cout << reach(adj, x - 1, y - 1);
}
