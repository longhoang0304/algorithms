#include <iostream>
#include <queue>
#include <vector>

using std::queue;
using std::vector;

int bipartite(vector<vector<int> > &adj) {
  vector<int> colours(adj.size(), -1);
  vector<int> visited(adj.size(), 0);
  queue<int> q;

  for (int i = 0; i < adj.size(); i++) {
    if (visited[i]) continue;
    q.push(i);
    while (!q.empty()) {
      int n = q.front();
      q.pop();
      if (visited[n]) continue;
      visited[n] = 1;

      int v = colours[n];
      if (v == -1) colours[n] = 0;
      v = colours[n];

      int nextcolour = int(!v);

      for (auto c : adj[n]) {
        if (colours[c] != -1 && colours[c] != nextcolour) return 0;
        if (visited[c]) continue;
        q.push(c);
        if (colours[c] == -1) colours[c] = nextcolour;
      }
    }
  }

  return 1;
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
  std::cout << bipartite(adj);
}
