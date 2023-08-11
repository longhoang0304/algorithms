#include <stdio.h>

typedef long long ll;

int main(void) {
  int n;
  scanf("%d", &n);
  ll ele;
  ll max = -1;
  ll max2 = -1;
  for (int i = 0; i < n; i++) {
    scanf("%lld", &ele);
    if (ele > max) {
      max2 = max;
      max = ele;
      continue;
    }
    if (ele > max2) max2 = ele;
  }
  // printf("%d %d\n", max, max2);
  ll res = max * max2;
  printf("%lld", res);
}
