

void reverseWord(char *s, char* e, int l) {
    int k = l / 2;
    while(k--) {
        *s ^= *e ^= *s ^= *e;
        s++;
        e--;
    }
}

char * reverseWords(char * str){
    int l = 1;
    char *s = str;
    char *e = str;
    while(*e) {
        if (*e == ' ') {
            reverseWord(s, e - 1, l - 1);
            l = 0;
            s = e + 1;
        }
        e++; l++;
    }
    reverseWord(s, e - 1, l - 1);
    return str;
}
