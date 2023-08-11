#include <stdio.h>

typedef long long ll;

ll gcd(ll a, ll b) {
  if (a < b) {
    ll tmp = a;
    a = b;
    b = tmp;
  }
  if (b == 0) return a;
  gcd(b, a % b);
}

int main(void) {
  ll a, b;
  scanf("%lld %lld", &a, &b);
  printf("%lld", gcd(a, b));
  return 0;
}
