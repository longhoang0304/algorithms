#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <ctype.h>
#include <math.h>

#define MAX(a, b) (a) > (b) ? (a) : (b)
#define MIN(a, b) (a) < (b) ? (a) : (b)

char* split_num(char* num, size_t length) {
    if (length == 0) {
        char *str = (char *)malloc(sizeof(char) * 2);
        str[0] = '0';
        str[1] = 0;
        return str;
    }
    char *str = (char *)malloc(sizeof(char) * (length + 1));
    strncpy(str, num, length);
    return str;
}

/**
 * Shift a number by length
 * E.g: num = 123, shift = 3 -> 123000
 */
char* shift_num(char* num, size_t length) {
    size_t lr = strlen(num) + length;
    char *res = (char *)calloc(lr + 1, sizeof(char));

    return res;
}

char* reverse_str(char *str)  {
    size_t ls = strlen(str);
    char* rev_str = (char *)calloc(ls + 1, sizeof(char));
    int i = ls - 1;
    for(; i >= 0; i--) {
        rev_str[ls - i - 1] = str[i];
    }
    rev_str[ls] = 0;
    return rev_str;
}

char* add_num(char *a, char *b) {
    size_t la = strlen(a);
    size_t lb = strlen(b);
    size_t lr = MAX(la, lb) + 1; // +1 for case 9 + 3
    int c = 0;
    int i = la - 1;
    int j = lb - 1;
    int k = 0;
    char *res = (char*)calloc(lr + 1, sizeof(char));

    while(i >= 0 || j >= 0) {
        int na = i < 0 ? 0 : a[i] - '0';
        int nb = j < 0 ? 0 : b[j] - '0';
        res[k] = ((na + nb + c) % 10) + '0';
        
        c = (na + nb + c) / 10;
        i--; j--; k++;
    }

    if (c) {
        res[k++] = '1';
    }

    res[k] = 0;

    // reverse result so that we can get correct result
    char *final_res = reverse_str(res);
    free(res);

    return final_res;
}

char* karatsuba(char *x, char *y) {
    size_t lx = strlen(x);
    size_t ly = strlen(y);
    char *res = (char *)malloc(sizeof(char) * (lx + ly + 1));

    // base case
    if (lx == 1 && ly == 1) {
        int r = (x[0] - '0') * (y[0] - '0');
        sprintf(res, "%d", r);
        return res;
    }

    // shit get real here
    // preparation
    size_t la = ceil(lx / 2.0);
    size_t lb = floor(lx / 2.0);
    size_t lc = ceil(ly / 2.0);
    size_t ld = floor(ly / 2.0);

    char *a = split_num(x,      la);
    char *b = split_num(x + la, lb);
    char *c = split_num(y,      lc);
    char *d = split_num(y + lc, ld);

    printf("a=%s b=%s c=%s d=%s", a, b, c, d);
    

    // end
    free(a); free(b); free(c); free(d);

    return res;
}

int main(void) {
    // char *res = karatsuba("0", "1234567");
    // printf("%s\n", res);
    // free(res);

    char *res = add_num("9", "9");
    printf("%s\n", res);
    free(res);

    return 0;
}