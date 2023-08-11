#include <stdio.h>

int main(void) {
  int t;
  scanf("%d", &t);
  int s, d, k;
  int i = 1;
  while (t--) {
    scanf("%d %d %d", &s, &d, &k);
    int buns = 2 * (s + d);
    int patties = s + 2 * d;
    if (k + 1 <= buns && k <= patties) {
      printf("Case #%d: YES\n", i);
    } else {
      printf("Case #%d: NO\n", i);
    }
    i += 1;
  }
  return 0;
}