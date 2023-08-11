#include <stdio.h>

#define MAX(a, b) (a) > (b) ? (a) : (b)
#define MIN(a, b) (a) < (b) ? (a) : (b)

const int COINS[3] = {1, 3, 4};


int main(int argc, char const *argv[])
{
  int n;
  scanf("%d", &n);
  int change[n + 1];
  change[0] = 0;
  for(int i = 1; i <= n; i++) {
    int min_coins = 0x7FFFFFFF;
    for (int c = 0; c < 3; c++) {
      int prev_change = MAX(i - COINS[c], 0);
      int curr_coins = change[prev_change] + 1;
      if (min_coins > curr_coins) min_coins = curr_coins;
    }
    change[i] = min_coins;
  }
  printf("%d", change[n]);
  return 0;
}
