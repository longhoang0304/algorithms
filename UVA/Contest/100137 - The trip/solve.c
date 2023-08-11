#include <stdio.h>
#include <stdlib.h>
#include <math.h>

double round(double v) {
	double nv = v * 100;
	nv = ceil(nv);
	nv /= 100;
	return nv;
}

void solve(double *a, int n) {
    double rn = 0;
    double rp = 0;
    double avg = 0;
    double s = 0;

    int i;
    for(i = 0; i < n; i++) {
        s += a[i];
    }

    avg = round(s / (n * 1.0));
    for(i = 0; i < n; i++) {
        if (a[i] > avg) {
            rn += (a[i] - avg);
            continue;
        }
        rp += (avg - a[i]);
    }

    printf("$%.2lf\n", rn > rp ? rp : rn);
}

int main(void) {
    int n, i;
    double *a;
    while (scanf("%d", &n) == 1 && n != 0) {
        a = (double *)malloc(n * sizeof(double));
        for (i = 0; i < n; i++) {
            scanf("%lf", &a[i]);
        }
        solve(a, n);
        free(a);
    }
    return 0;
}
