#include <algorithm>
#include <iostream>
#include <set>
#include <stack>
#include <vector>

using std::pair;
using std::set;
using std::sort;
using std::stack;
using std::vector;

typedef pair<int, int> pii;
typedef vector<vector<int>> graph;

vector<pii> calPostNumber(graph adj) {
  vector<int> preNumbers = vector<int>(adj.size(), 0);
  vector<int> postNumbers = vector<int>(adj.size(), 0);
  int number = 1;

  vector<int> visited(adj.size(), 0);
  stack<int> st = stack<int>();
  for (int i = adj.size() - 1; i >= 0; i--) {
    st.push(i);
  }

  while (!st.empty()) {
    int n = st.top();

    if (visited[n]) {
      st.pop();
      if (postNumbers[n] == 0) {
        postNumbers[n] = number;
        number += 1;
      }
      continue;
    }

    visited[n] = 1;
    preNumbers[n] = number;
    number += 1;
    for (auto c : adj[n]) {
      if (visited[c]) continue;
      st.push(c);
    }
  }
  vector<pii> postNumberResuls = vector<pii>(postNumbers.size());
  for (int i = 0; i < postNumbers.size(); i++) {
    postNumberResuls[i] = pii(i, postNumbers[i]);
  }
  return postNumberResuls;
}

graph reverseGraph(graph adj) {
  graph reversed = graph(adj.size(), vector<int>());
  for (int i = 0; i < adj.size(); i++) {
    for (int j = 0; j < adj[i].size(); j++) {
      reversed[adj[i][j]].push_back(i);
    }
  }
  return reversed;
}

void dfs(int root, graph &adj, vector<int> &used) {
  stack<int> st;
  st.push(root);

  while (!st.empty()) {
    int n = st.top();
    st.pop();
    if (used[n]) continue;
    used[n] = 1;
    for (auto c : adj[n]) {
      if (used[c]) continue;
      st.push(c);
    }
  }
}

int number_of_strongly_connected_components(graph &adj) {
  graph reversed = reverseGraph(adj);
  vector<pii> postNumbers = calPostNumber(reversed);
  sort(postNumbers.begin(), postNumbers.end(), [](pii a, pii b) { return a.second > b.second; });
  vector<int> used(adj.size(), 0);
  int result = 0;

  for (auto p : postNumbers) {
    if (used[p.first]) continue;
    result += 1;
    dfs(p.first, adj, used);
  }

  return result;
}

int main() {
  size_t n, m;
  std::cin >> n >> m;
  vector<vector<int>> adj(n, vector<int>());
  for (size_t i = 0; i < m; i++) {
    int x, y;
    std::cin >> x >> y;
    adj[x - 1].push_back(y - 1);
  }
  std::cout << number_of_strongly_connected_components(adj);
}
