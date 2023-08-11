#include <stdio.h>
#include <string.h>

#define MAX(a, b) (a) > (b) ? (a) : (b)
#define MIN(a, b) (a) < (b) ? (a) : (b)

int main(int argc, char const *argv[]) {
  char s1[101];
  char s2[101];

  scanf("%[^\n]%*c", s1);
  scanf("%[^\n]", s2);

  int s1_length = strlen(s1) + 1;
  int s2_length = strlen(s2) + 1;
  int map[s1_length][s2_length];

  for (int i = 0; i < s1_length; i++) {
    map[i][0] = i;
  }
  for (int i = 0; i < s2_length; i++) {
    map[0][i] = i;
  }

  for (int j = 1; j < s2_length; j++) {
    for (int i = 1; i < s1_length; i++) {
      map[i][j] = MIN(map[i][j - 1] + 1, map[i - 1][j] + 1);
      if (s1[i - 1] == s2[j - 1]) {
        map[i][j] = MIN(map[i][j], map[i - 1][j - 1]);
      } else {
        map[i][j] = MIN(map[i][j], map[i - 1][j - 1] + 1);
      }
    }
  }
  printf("%d", map[s1_length - 1][s2_length - 1]);
  return 0;
}
