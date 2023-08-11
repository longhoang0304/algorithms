#include <stdio.h>

int collatz(int n) {
    int step = 1;
    while(n != 1) {
        if (n % 2) n = n * 3 + 1;
        else n /= 2;
        step += 1;
    }

    return step;
}

void solve(int i, int j) {
    int max = -1;
    int k = i;
    while(k <= j) {
        int res = collatz(k);
        if (max < res) {
            max = res;
        }
        k += 1;
    }
    printf("%d %d %d\n", i, j, max);
}

int main(void) {
    int i, j;
    while (scanf("%d %d", &i, &j) == 2) {
        solve(i, j);
    }
    return 0;
}
