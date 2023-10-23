#include <algorithm>
#include <iostream>
#include <stack>
#include <vector>

using std::cin;
using std::cout;
using std::endl;
using std::stack;
using std::vector;

struct Node {
  int key;
  int left;
  int right;

  Node() : key(0), left(-1), right(-1) {}
  Node(int key_, int left_, int right_) : key(key_), left(left_), right(right_) {}
};

bool IsBinarySearchTree(const vector<Node>& Tree) {
  vector<Node> tree = Tree;

  if (tree.size() < 1) return true;

  int curr = 0;
  int prev = -1;

  // morris inorder traversal
  while (curr != -1) {
    Node n = tree[curr];
    Node pn = tree[prev];

    if (n.left == -1) {
      if (prev != -1 && prev != curr && pn.key >= n.key) return false;
      prev = curr;
      curr = n.right;
      continue;
    }

    prev = n.left;
    pn = tree[prev];
    while (pn.right != -1 && pn.right != curr) {
      prev = pn.right;
      pn = tree[prev];
    }

    if (pn.right == -1) {
      tree[prev].right = curr;
      curr = n.left;
      continue;
    }

    pn.right = -1;
    if (pn.key >= n.key) return false;
    prev = curr;
    curr = n.right;
  }

  return true;
}

int main() {
  int nodes;
  cin >> nodes;
  vector<Node> tree;
  for (int i = 0; i < nodes; ++i) {
    int key, left, right;
    cin >> key >> left >> right;
    tree.push_back(Node(key, left, right));
  }
  if (IsBinarySearchTree(tree)) {
    cout << "CORRECT" << endl;
  } else {
    cout << "INCORRECT" << endl;
  }
  return 0;
}
