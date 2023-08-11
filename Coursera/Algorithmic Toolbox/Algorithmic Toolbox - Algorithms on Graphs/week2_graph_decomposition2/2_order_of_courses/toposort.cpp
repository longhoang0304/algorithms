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

vector<int> toposort(vector<vector<int>> adj) {
  vector<int> used(adj.size(), 0);
  vector<int> order = vector<int>();
  
  stack<int> st;
  for (int i = adj.size() - 1; i >= 0; i--) {
    st.push(i);
  }

  while (!st.empty()) {
    int n = st.top();
    int hasChild = 0;
    if (used[n]) {
        st.pop();
        continue;
    }
    for (auto c : adj[n]) {
      if (used[c]) continue;
      hasChild = 1;
      st.push(c);
    }
    if (!hasChild) {
      st.pop();
      order.push_back(n);
      used[n] = 1;
    }
  }

  std::reverse(order.begin(), order.end());

  return order;
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
  vector<int> order = toposort(adj);
  for (size_t i = 0; i < order.size(); i++) {
    std::cout << order[i] + 1 << " ";
  }
}
