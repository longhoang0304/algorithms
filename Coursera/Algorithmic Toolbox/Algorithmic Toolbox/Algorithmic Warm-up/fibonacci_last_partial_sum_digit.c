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
  ll n, m;
  ll res[2] = {0, 0};

  scanf("%lld %lld", &m, &n);

  m -= 1;
  fast_doubling(m + 2, res, 10);
  ll fm = res[0] - 1;


  res[0] = 0;
  res[1] = 0;
  fast_doubling(n + 2, res, 10);
  ll fn = res[0] - 1;

  printf("%lld", (fn - fm + 10) % 10);
  return 0;
}
