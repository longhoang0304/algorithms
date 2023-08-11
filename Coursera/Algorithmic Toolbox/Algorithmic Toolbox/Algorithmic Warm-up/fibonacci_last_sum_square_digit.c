#include <stdio.h>

typedef long long ll;

void fast_doubling(ll n, ll res[2], ll m) {
  if (!n) {
    res[0] = 0;
    res[1] = 1;
    return;
  }

  fast_doubling(n / 2, res, m);

  ll fn = res[0];
  ll fn1 = res[1];
  ll f2n = 2 * fn1 - fn;

  if (f2n < 0) f2n += m;
  f2n = (fn * f2n) % m;

  ll f2n1 = (fn * fn + fn1 * fn1) % m;

  if (n % 2 == 0) {
      res[0] = f2n;
      res[1] = f2n1;
  }
  else {
      res[0] = f2n1;
      res[1] = f2n + f2n1;
  }

  res[0] %= m;
  res[1] %= m;
}
 

int main(void) {
  ll n;
  ll res[2] = {0, 0};

  scanf("%lld", &n);

  if (n < 2) {
    printf("%lld", n != 0);
    return 0;
  }

  fast_doubling(n - 1, res, 10);
  ll fn1 = res[0];
  ll fn = res[1];

  printf("%lld", ((fn1 + fn) * fn) % 10);
  return 0;
}
