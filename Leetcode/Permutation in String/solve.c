

bool checkInclusion(char * s1, char * s2){
    int *default_map = (int *)calloc(sizeof(int), 26);
    bool *char_map = (bool *)calloc(sizeof(bool), 26);
    int default_length = strlen(s1);
    int s1_length = strlen(s1);
    int i = 0;
    for (i = 0; i < s1_length; i++) {
        int k = (int)(s1[i] - 'a');
        default_map[k] += 1;
        char_map[k] = true;
    }
    
    int *map = (int *)calloc(sizeof(int), 26);
    char **first_meet = (char **)calloc(sizeof(char *), 26);
    memcpy(map, default_map, sizeof(int) * 26);
    int length = default_length;
    char *s = s2;
    while (*s) {
        int k = (int)(*s - 'a');
        if (!length) return true;
        if (char_map[k] && map[k]) {
            if (!first_meet[k]) first_meet[k] = s;
            map[k] -= 1;
            length -= 1;
            s++;
            continue;
        }
        if (char_map[k] && first_meet[k]) s = first_meet[k];
        if (length != default_length) {
            memset(first_meet, 0, sizeof(char *) * 26);
            length = default_length;
            memcpy(map, default_map, sizeof(int) * 26);   
        }
        s++;
    }
    return !length;
}