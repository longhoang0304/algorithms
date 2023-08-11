#include <stdio.h>

int main(void) {
  int f1 = 1;
  int f2 = 1;
  int n;
  scanf("%d", &n);
  if (n < 3) {
    printf("%d", n != 0);
    return 0;
  }
  n -= 2;
  while (n) {
    n -= 1;
    int f3 = f1 + f2;
    f1 = f2;
    f2 = f3;
  }
  printf("%d", f2);
  return 0;
}
