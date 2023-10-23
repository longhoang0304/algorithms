#include <iostream>
#include <queue>
#include <vector>

#define MIN(a, b) (a < b) ? (a) : (b)

using std::queue;
using std::vector;

int distance(vector<vector<int> > &adj, int s, int t) {
  vector<int> shortest(adj.size(), 99999999);
  vector<int> visited(adj.size(), 0);
  queue<int> q = queue<int>();
  q.push(s);
  shortest[s] = 0;
  while (!q.empty()) {
    int n = q.front();
    q.pop();
    int v = shortest[n];
    if (visited[n]) continue;
    visited[n] = 1;
    for (auto c : adj[n]) {
      if (visited[c]) continue;
      q.push(c);
      shortest[c] = MIN(v + 1, shortest[c]);
    }
  }
  if (!visited[t]) return -1;
  return shortest[t];
}

int main() {
  int n, m;
  std::cin >> n >> m;
  vector<vector<int> > adj(n, vector<int>());
  for (int i = 0; i < m; i++) {
    int x, y;
    std::cin >> x >> y;
    adj[x - 1].push_back(y - 1);
    adj[y - 1].push_back(x - 1);
  }
  int s, t;
  std::cin >> s >> t;
  s--, t--;
  std::cout << distance(adj, s, t);
}
