#include <iostream>
#include <vector>
#include <bits/stdc++.h>

using namespace std;

int optimal_weight(int W, const vector<int> &w) {
  //write your code here
  vector<int> values(w.size());
  vector<vector<int>>  arr(w.size() + 1,vector<int>(W + 1));
  for(int i=0;i<w.size();i++){
    values[i]=w[i];
  }

  for(int i=0;i<=values.size();i++){
    for(int j=0;j<=W;j++){
      if(i==0 || j==0)
        arr[i][j]=0;
      else if(w[i-1]<=j){
        arr[i][j]=max(values[i-1]+arr[i-1][j-w[i-1]],arr[i-1][j]);
      } else{
        arr[i][j]=arr[i-1][j];
      }
    }
  }

  return arr[w.size()][W];

}

int main() {
  int n, W;
  cin >> W >> n;
  vector<int> w(n);
  for (int i = 0; i < n; i++) {
    cin >> w[i];
  }
  cout << optimal_weight(W, w) << '\n';
}