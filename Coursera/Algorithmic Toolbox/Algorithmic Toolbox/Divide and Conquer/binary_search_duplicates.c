#include <stdio.h>

#define MIN(a, b) (a) < (b) ? (a) : (b)
#define MAX(a, b) (a) > (b) ? (a) : (b)

int bin_search(int* arr, int target, int arr_size) {
  int left = 0;
  int right = arr_size - 1;
  while (right >= left) {
    int middle = (right + left) / 2;
    int lval = arr[MAX(middle - 1, 0)];
    int val = arr[middle];
    if (val == target && (lval != val || !middle)) return middle;
    if (val == target && lval == val) right = MAX(middle - 1, 0);
    if (val < target) left = middle + 1;
    if (val > target) right = middle - 1;
  }
  return -1;
}

int main(int argc, char const *argv[]) {
  int n, m, target;
  scanf("%d", &n);
  int arr[n];
  for (int i = 0; i < n; i++) {
    scanf("%d", &arr[i]);
  }
  scanf("%d", &m);
  while(m--) {
    scanf("%d", &target);
    printf("%d ", bin_search(arr, target, n));
  }
  return 0;
}
