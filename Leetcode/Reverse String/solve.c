

void reverseString(char* str, int sSize){
    char *s = str;
    char *e = str + (sSize - 1);
    int l = sSize >> 1;
    while(l--) {
        *s ^= *e ^= *s ^= *e;
        s++;
        e--;
    }
}