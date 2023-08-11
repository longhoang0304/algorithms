#include <stdio.h>


int main(void) {
  int value;
  int res = 0;
  int money[3] = {10, 5, 1};
  scanf("%d", &value);
  for (int i = 0; i < 3; i++) {
    int v = value / money[i];
    value -= v * money[i];
    res += v;
  }
  printf("%d", res);
}