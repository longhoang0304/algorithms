#include <iostream>
#include <vector>
#include <algorithm>
#include <limits.h>

using std::vector;
using std::cin;
using std::cout;
using std::swap;
using std::pair;
using std::make_pair;

class HeapBuilder {
 private:
  vector<int> data_;
  vector< pair<int, int> > swaps_;

  void WriteResponse() const {
    cout << swaps_.size() << "\n";
    for (int i = 0; i < swaps_.size(); ++i) {
      cout << swaps_[i].first << " " << swaps_[i].second << "\n";
    }
  }

  void ReadData() {
    int n;
    cin >> n;
    data_.resize(n);
    for(int i = 0; i < n; ++i)
      cin >> data_[i];
  }

  void SiftDown(int idx) {
    int root = data_[idx];
    int size = data_.size();
    int lidx = 2 * (idx + 1) - 1;
    int ridx = 2 * (idx + 1);
    // cout << "idx: " << idx << " " << lidx << " " << ridx << std::endl;
    int left = lidx < size ? data_[lidx] : INT_MAX;
    int right = ridx < size ? data_[ridx] : INT_MAX;

    if (left < right) {
      if (root <= left) return;

      data_[idx] = left;
      data_[lidx] = root;
      swaps_.push_back(pair<int, int>(idx, lidx));
      SiftDown(lidx);
      return;
    }

    if (root <= right) return;

    data_[idx] = right;
    data_[ridx] = root;
    swaps_.push_back(pair<int, int>(idx, ridx));
    SiftDown(ridx);
    return;

  }

  void GenerateSwaps() {
    swaps_.clear();
    // The following naive implementation just sorts 
    // the given sequence using selection sort algorithm
    // and saves the resulting sequence of swaps.
    // This turns the given array into a heap, 
    // but in the worst case gives a quadratic number of swaps.
    //
    // TODO: replace by a more efficient implementation
    for (int i = data_.size() / 2; i >= 0 ; i--) SiftDown(i);
  }

 public:
  void Solve() {
    ReadData();
    GenerateSwaps();
    WriteResponse();
  }
};

int main() {
  std::ios_base::sync_with_stdio(false);
  HeapBuilder heap_builder;
  heap_builder.Solve();
  return 0;
}
