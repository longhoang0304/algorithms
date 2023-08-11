#include <stdio.h>

#define MAX(a, b) (a) > (b) ? (a) : (b)
#define MIN(a, b) (a) < (b) ? (a) : (b)

int main(int argc, char const *argv[]) {
  int n;
  scanf("%d", &n);
  int ops[n + 1];
  ops[0] = 0;
  ops[1] = 0;
  for (int i = 2; i <= n; i++) {
    int min_ops = 0x7FFFFFFF;

    int prev_value = MAX(i - 1, 0);
    int curr_ops = ops[prev_value] + 1;
    if (curr_ops < min_ops) {
      min_ops = curr_ops;
    }

    if (i % 2 == 0) {
      prev_value = i / 2;
      curr_ops = ops[prev_value] + 1;
      if (curr_ops < min_ops) {
        min_ops = curr_ops;
      }
    }

    if (i % 3 == 0) {
      prev_value = i / 3;
      curr_ops = ops[prev_value] + 1;
      if (curr_ops < min_ops) {
        min_ops = curr_ops;
      }
    }
    ops[i] = min_ops;
  }
  
  int min_values[n];
  int min_idx = 0;
  min_values[min_idx] = n;
  min_idx += 1;
  int current_value = min_values[min_idx - 1];
  while (current_value > 1) {
    current_value = min_values[min_idx - 1];
    int current_ops = ops[current_value];
    int min_ops = 0x7FFFFFFF;
    int min_value;

    if (ops[current_value - 1] < min_ops) {
      min_value = current_value - 1;
      min_ops = ops[min_value];
    }

    if (current_value % 2 == 0) {
      if (ops[current_value / 2] < min_ops) {
        min_value = current_value / 2;
        min_ops = ops[min_value];
      }
    }

    if (current_value % 3 == 0) {
      if (ops[current_value / 3] < min_ops) {
        min_value = current_value / 2;
        min_ops = ops[min_value];
      }
    }

    min_values[min_idx++] = min_value;
  };

  printf("%d %d\n", ops[n], min_idx);
  while(min_idx) {
    min_idx -= 1;
    printf("%d ", min_values[min_idx]);
  }
  return 0;
}
