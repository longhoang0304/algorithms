// The API isBadVersion is defined for you.
// bool isBadVersion(int version);

bad_version = 1

int isBadVersion(int version) {
  return version >= bad_version;
}

int firstBadVersion(int n) {
    long long l = 0;
    long long r = n;
    long long  m = (l + r) / 2;
    long long min = 2147483647;
    while (l <= r) {
        if (isBadVersion(m)) {
            if (m < min) {
                min = m;
            }
            r = m - 1;
        } else {
            l = m + 1;
        }
        m = (l + r) / 2;
    }
    return min;
}
