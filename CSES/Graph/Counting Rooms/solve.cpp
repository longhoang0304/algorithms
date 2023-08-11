#include <iostream>
#include <vector>
#include <stack>
#include <algorithm>
#include <utility>

using namespace std;

void dfs(pair<int, int> node, vector<vector<bool>> &map, vector<vector<bool>> &visited) {
    int X[4] = {0, 1, 0, -1};
    int Y[4] = {-1, 0, 1, 0};
    int mX = map[0].size();
    int mY = map.size();

    stack<pair<int, int>> nodeList;
    nodeList.push(node);
    visited[node.first][node.second] = true;

    while(!nodeList.empty()) {
        pair<int, int> cNode = nodeList.top();
        nodeList.pop();

        for (int i = 0; i < 4; i++) {
            if (cNode.second + X[i] < 0) continue;
            if (cNode.second + X[i] >= mX) continue;
            if (cNode.first + Y[i] < 0) continue;
            if (cNode.first + Y[i] >= mY) continue;

            pair<int, int> newNode(cNode.first + Y[i], cNode.second + X[i]);
            if (!map[newNode.first][newNode.second]) continue;
            if (visited[newNode.first][newNode.second]) continue;

            visited[newNode.first][newNode.second] = true;

            nodeList.push(newNode);
        }
    }
}

int solve(vector<pair<int, int>> &floors, vector<vector<bool>> &map) {
    int res = 0;
    int fLength = floors.size();
    vector<vector<bool>> visited(map.size(), vector<bool>(map[0].size(), false));

    for (int i = 0; i < fLength; i++) {
        pair<int, int> current = floors[i];
        // visited node, skip
        if (visited[current.first][current.second]) {
            continue;
        }

        // new room discovered
        res += 1;
        // mark all the same floor
        dfs(current, map, visited);
    }

    return res;
}

int main(void) {
    int n, m;
    char c;

    cin >> n >> m;

    vector<pair<int, int>> floors;
    vector<vector<bool>> map(n, vector<bool>(m, false));

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            cin >> c;
            if (c == '.') {
                map[i][j] = true;
                floors.push_back(make_pair(i, j));
                continue;
            }
        }
        cin.ignore(256, '\n');
    }

    int res = solve(floors, map);
    cout << res;
}
