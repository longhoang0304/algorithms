#include <iostream>
#include <vector>
#include <map>
#include <stack>
#include <queue>
#include <algorithm>
#include <utility>

using namespace std;

string stepDir = "URDL";
bool V[1000][1000];
int M[1000][1000];
int S[1000][1000];
int n, m;
    int X[4] = {0, 1, 0, -1};
    int Y[4] = {-1, 0, 1, 0};

void bfs(pair<int, int> start) {
    int mX = m;
    int mY = n;
    bool possible = false;

    queue<pair<int, int>> nodeList;
    nodeList.push(start);
    V[start.first][start.second] = true;

    while(!nodeList.empty()) {
        pair<int, int> cNode = nodeList.front(); nodeList.pop();

        for (int i = 0; i < 4; i++) {
            if (cNode.second + X[i] < 0) continue;
            if (cNode.second + X[i] >= mX) continue;
            if (cNode.first + Y[i] < 0) continue;
            if (cNode.first + Y[i] >= mY) continue;

            pair<int, int> newNode(cNode.first + Y[i], cNode.second + X[i]);

            if (!M[newNode.first][newNode.second]) continue;
            if (V[newNode.first][newNode.second]) continue;

            S[newNode.first][newNode.second] = i;
            V[newNode.first][newNode.second] = true;

            if (M[newNode.first][newNode.second] == 2) return;
            nodeList.push(newNode);
        }
    }
    return;
}

void solve(pair<int, int> start, pair<int, int> end) {
    bfs(start);
    if (!V[end.first][end.second]) {
        cout << "NO";
        return;
    }
    cout << "YES" << endl;
    string res = "";

    pair<int, int> node = end;
    vector<int> steps;

    while(true) {
        int step = S[node.first][node.second];
        steps.push_back(step);

        node.first -= Y[step];
        node.second -= X[step];

        if (node.first == start.first && node.second == start.second) break;
    }

    reverse(steps.begin(), steps.end());

    cout << steps.size() << endl;
    for (int c : steps) {
        cout << stepDir[c];
    }
}

int main(void) {
    char c;

    cin >> n >> m;

    pair<int, int> start;
    pair<int, int> end;

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            cin >> c;
            if (c == '.' || c == 'A' || c == 'B') {
                M[i][j] = 1;
            }
            if (c == 'A') {
                start.first = i;
                start.second = j;
            }
            if (c == 'B') {
                M[i][j] = 2;
                end.first = i;
                end.second = j;
            }
        }
    }

    solve(start, end);
}
