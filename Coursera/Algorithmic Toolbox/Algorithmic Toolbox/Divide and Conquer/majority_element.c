#include <stdio.h>
#include <stdlib.h>

int cmp(const void *a, const void *b) {
  return *(int *)a - *(int *)b;
}

int main(int argc, char const *argv[]) {
  int n;
  scanf("%d", &n);
  int arr[n];
  for (int i = 0; i < n; i++) {
    scanf("%d", &arr[i]);
  }
  qsort(arr, n, sizeof(int), cmp);
  int c = 0;
  int prev = -1;
  int res = 0;
  for (int i = 0; i < n; i++) {
    if (arr[i] != prev) {
      prev = arr[i];
      c = 0;
    }
    c += 1;
    if (c > (n >> 1)) {
      res = 1;
    }
  }
  printf("%d", res);
  return 0;
}
