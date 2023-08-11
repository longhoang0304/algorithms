#include <iostream>
#include <queue>
#include <cmath>

using namespace std;

typedef pair<int, int> value;
typedef pair<value, int> node;
typedef queue<node> node_list;

int solve(int a, int b) {
  node_list *q = new node_list();
  int res = 0;

  q->push(node(value(a, b), 0));

  while (!q->empty()) {
    node n = q->front();
    value v = n.first;
    int depth = n.second + 1;
    int a = v.first;
    int b = v.second;

    q->pop();

    if (a < b) {
      res = depth;
      break;
    }

    q->push(node(value(floor(a * 1.0 / b), b), depth));
    q->push(node(value(a, b + 1), depth));
  }
  return res;
}

int main(void) {
  int t;
  int a, b;
  cin >> t;
  while(t--) {
    cin >> a >> b;
    cout << solve(a, b) << endl;
  }
}