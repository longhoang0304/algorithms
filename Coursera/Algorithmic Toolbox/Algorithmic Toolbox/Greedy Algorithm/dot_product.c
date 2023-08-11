#include <stdio.h>
#include <stdlib.h>

typedef long long ll;

int cmp(void *a, void *b) {
  return *(ll *)a - *(ll *) b;
}

int main(void) {
  int n;
  scanf("%d", &n);

  ll first[n];
  ll second[n];
  ll res = 0;

  for (int i = 0; i < n; i++) {
    scanf("%lld", &first[i]);
  }
  for (int i = 0; i < n; i++) {
    scanf("%lld", &second[i]);
  }

  qsort(first, n, sizeof(ll), cmp);
  qsort(second, n, sizeof(ll), cmp);

  for (int i = 0; i < n; i++) {
    res += first[i] * second[i];
  }

  printf("%lld", res);
}