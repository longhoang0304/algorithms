#include <stdio.h>

#define MAX(a, b) (a) > (b) ? (a) : (b)
#define MIN(a, b) (a) < (b) ? (a) : (b)

int main(int argc, char const *argv[])
{
  int n;
  scanf("%d", &n);
  int values[n];
  int sum = 0;
  for(int i = 0; i < n; i++) {
    scanf("%d ", &values[i]);
    sum += values[i];
  }
  if (sum % 3 != 0) {
    puts("0");
    return 0;
  }
  int dp[sum + 1][sum + 1];
  dp[0][0] = 1;
  for (int i = 0; i < n; i++) {
    for (int j = sum; j >= 0; j--) {
      for (int k = sum; k >= 0; k--) {
        if (dp[j][k]) {
          dp[j + values[i]][k] = 1;
          dp[j][k + values[i]] = 1;
        }
      }  
    }
  }

  printf("%d", dp[sum / 3][sum / 3]);
  return 0;
}
