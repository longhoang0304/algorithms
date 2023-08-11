#include <iostream>
#include <cassert>
#include <string>
#include <vector>
#include<cstring>
#include<limits.h>

using namespace std;

long long eval(long long a, long long b, char op) {
  if (op == '*') {
    return a * b;
  } else if (op == '+') {
    return a + b;
  } else if (op == '-') {
    return a - b;
  } else {
    assert(0);
  }
}

long long get_maximum_value(const string &exp) {
  int expLen = exp.size();
	int NumOfOperands = (expLen + 1) / 2;

	long long Min[NumOfOperands][NumOfOperands];
	long long Max[NumOfOperands][NumOfOperands];

	memset(Min, 0, sizeof(Min));
	memset(Max, 0, sizeof(Max));

	for (int i = 0; i < NumOfOperands; i++) {
		Min[i][i] = stoll(exp.substr(2 * i, 1));
		Max[i][i] = stoll(exp.substr(2 * i, 1));
	}

	for (int s = 0; s < NumOfOperands - 1; s++) {
		for (int i = 0; i < NumOfOperands - s - 1; i++) {
			int j = i + s + 1;
			long long MinValue = LLONG_MAX;
			long long MaxValue = LLONG_MIN;

			for (int k = i; k < j; k++) {
				long long a = eval(Min[i][k], Min[k + 1][j], exp[2 * k + 1]);
				long long b = eval(Min[i][k], Max[k + 1][j], exp[2 * k + 1]);
				long long c = eval(Max[i][k], Min[k + 1][j], exp[2 * k + 1]);
				long long d = eval(Max[i][k], Max[k + 1][j], exp[2 * k + 1]);

				MinValue = min(MinValue, min(a, min(b, min(c, d))));
				MaxValue = max(MaxValue, max(a, max(b, max(c, d))));

			}
			Min[i][j] = MinValue;
			Max[i][j] = MaxValue;
		}
	}

	return Max[0][NumOfOperands - 1];
}

int main() {
  string s;
  std::cin >> s;
  std::cout << get_maximum_value(s) << '\n';
}