#include <stdio.h>

#define MAX(a, b) (a) > (b) ? (a) : (b)
#define MIN(a, b) (a) < (b) ? (a) : (b)

  int res[100000][100000];

int main(int argc, char const *argv[])
{
  int w, n;
  scanf("%d %d", &w, &n);
  int golds[n + 1];
  res[0][0] = 0;
  for(int i = 0; i < n; i++) {
    res[0][i] = 0;
    scanf("%d ", &golds[i + 1]);
  }
  for(int i = 0; i <= w; i++) {
    res[i][0] = 0;
  }
  res[0][n] = 0;

  for(int i = 1; i <= w; i++) {
    for (int j = 1; j <= n; j++) {
      res[i][j] = res[i][j - 1];
      if (golds[j] > i) {
        continue;
      }
      int val = res[i - golds[j]][j - 1] + golds[j];
      if (res[i][j] < val) res[i][j] = val;
    }
  }

  printf("%d", res[w][n]);
  
  return 0;
}
