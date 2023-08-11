#include <stdio.h>
#include <stdlib.h>

typedef struct Item {
  int weight;
  int value;
  double vpw;
} Item;

int cmp(void *a, void *b) {
  Item *ia = (Item *)a;
  Item *ib = (Item *)b;
  if (ib->vpw > ia->vpw) return 1;
  if (ib->vpw < ia->vpw) return -1;
  return 0;
}

int main(void) {
  int n, w;
  scanf("%d %d", &n, &w);
  Item items[n];
  for (int i = 0; i < n; i++) {
    scanf("%d %d", &items[i].value, &items[i].weight);
    items[i].vpw = items[i].value / (items[i].weight * 1.0);
  }

  qsort(items, n, sizeof(Item), cmp);
  double res = 0;
  for (int i = 0; i < n; i++) {
    Item item = items[i];
    if (item.weight >= w) {
      res += w * item.vpw;
      break;
    }
    res += item.weight * item.vpw;
    w -= item.weight;
  }
  printf("%lf", res);
}