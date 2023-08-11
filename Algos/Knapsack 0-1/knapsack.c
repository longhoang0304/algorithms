#include <stdio.h>
#include <stdlib.h>

#define MAX(a, b) (a) > (b) ? (a) : (b)
#define MIN(a, b) (a) < (b) ? (a) : (b)

void print_knapsack_path(int **dp, int k, int *w, int n) {
    int *path = (int *)calloc(n, sizeof(int));
    int i, j;
    int ck = k;
    for (i = n; i > 0; i--) {
        if (dp[i][ck] != dp[i - 1][k - 1]) {
            if (ck - w[i - 1] < 0) continue;
            path[i - 1] = 1;
            ck -= w[i - 1];
        }
    }
    for (i = 0; i < n; i++) {
        printf("%d ", path[i]);
    }
}

void knapsack_01(int k, int *v, int *w, int n) {
    int i, j;

    // setup dp config
    int **dp = (int **)malloc(sizeof(int *) * (n + 1));
    for (i = 0; i <= n; i++) {
        dp[i] = (int *)malloc(sizeof(int) * (k + 1));
    }

    // init dp config
    for (i = 0; i <= k; i++) {
        dp[0][i] = 0;
    }

    // dp
    for (i = 1; i <= n; i++) {
        for(j = 0; j <= k; j++) {
            if (j - w[i - 1] >= 0) {
                dp[i][j] = MAX(dp[i - 1][j], v[i - 1] + dp[i - 1][j - w[i - 1]]);
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    printf("%d 0\n", dp[n][k]);
    print_knapsack_path(dp, k, w, n);

    for (i = 0; i <= n; i++) {
        free(dp[i]);
    }
    free(dp);
    dp = NULL;

    return;
}

int main(void) {
    int n, k;

    scanf("%d %d", &n, &k);

    int *v = (int *)malloc(sizeof(int) * n);
    int *w = (int *)malloc(sizeof(int) * n);

    int i;
    for(i = 0; i < n; i++) {
        scanf("%d %d", &v[i], &w[i]);
    }

    knapsack_01(k, v, w, n);
    free(v);
    free(w);

    v = w = NULL;
}
