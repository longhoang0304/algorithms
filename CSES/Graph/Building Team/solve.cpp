#include <iostream>
#include <vector>
#include <map>
#include <stack>
#include <queue>
#include <algorithm>
#include <utility>

using namespace std;

string stepDir = "URDL";

void dfs(int s, vector<vector<int>> &G, vector<bool> &V) {
    stack<int> nodes;
    nodes.push(s);

    while(!nodes.empty()) {
        int node = nodes.top(); nodes.pop();
        V[node] = true;

        for (auto &child : G[node]) {
            if (V[child]) continue;
            nodes.push(child);
        }
    }
}

vector<int> solve(int n, vector<vector<int>> &G, vector<pair<int, int>> &N) {
    vector<bool> V(n + 1, false);
    vector<int> res;
    for (pair<int, int> node : N) {
        // cout << node.first << endl;
        if (V[node.first]) continue;

        res.push_back(node.first);
        dfs(node.first, G, V);
    }

    for (int i = 1; i <= n; i++) {
        if (!V[i]) res.push_back(i);
    }

    return res;
}

int main(void) {
    int n, m;
    cin >> n >> m;
    vector<vector<int>> G(n + 1);
    vector<pair<int, int>> N;
    for (int i = 0; i < m; i++) {
        int a, b;
        cin >> a >> b;
        G[a].push_back(b);
        G[b].push_back(a);
        N.push_back(make_pair(a, b));
    }

    auto res = solve(n, G, N);
    int rs = res.size() - 1;

    cout << rs << endl;
    for (int i = 0; i < rs; i++) {
        cout << res[i] << " " << res[i + 1] << endl;
    }
}
