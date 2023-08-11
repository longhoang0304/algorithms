

int lengthOfLongestSubstring(char * s){
    int r = 0;
    int m = 0;
    int l = strlen(s);
    int *v = (int *)calloc(sizeof(int), 255);
    int i = 0;
    for (; i < l; i++) {
        int k = (int)s[i];
        if (!v[k]) {
            v[k] = i + 1;
            m += 1;
            continue;
        }
        r = m > r ? m : r;
        i = v[k];
        k = (int)s[i];
        free(v);
        v = (int *)calloc(sizeof(int), 255);
        m = 1;
        v[k] = i + 1;
    }
    free(v);
    r = m > r ? m : r;
    return r;
}