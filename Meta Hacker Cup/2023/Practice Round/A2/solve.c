#define _CRT_SECURE_NO_WARNINGS

#include <stdio.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) < (b) ? (b) : (a))

typedef long long ll;

ll K(ll s, ll d) {
    ll patties = d * 2 + s;
    ll buns = 2 * (d + s);
    
    // printf("%16lld %16lld %16lld %16lld\n", s, d, patties, buns);

    if (patties < buns) return patties;
    if (patties == buns) return patties - buns % 2;

    return buns - 1;
}

ll solve(ll a, ll b, ll c) {
  if (c < a && c < b) return 0;
  if (c == MIN(a, b)) return 1;
  ll ab = a + b;
  ll all_singles = K(c / a, (c % a) / b);
  ll all_doubles = K((c % b) / a, c / b);
  ll all_sd = (c >= ab ? 3 * (c / ab) + solve(a, b, c % ab) : 0);
  return MAX(all_singles, MAX(all_doubles, all_sd));
}

int main(void) {
  int t;
  scanf("%d", &t);
  ll a, b, c;
  int i = 1;
  while (t--) {
    scanf("%lld %lld %lld", &a, &b, &c);
    printf("Case #%d: %lld\n", i, solve(a, b, c));
    i += 1;
  }
  return 0;
}
