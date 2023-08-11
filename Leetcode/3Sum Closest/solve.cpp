#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int threeSumClosest(vector<int>& n, int t) {
        sort(n.begin(), n.end());
        int nl = n.size();
        int resMin = -9999999;
        int resMax =  9999999;

        for (int i = 0; i < nl; i++) {
            int l = i + 1;
            int r = nl - 1;
            
            while (l < r) {
                int s = n[l] + n[r] + n[i];
                if (s < t) {
                    resMin = max(resMin, s);
                    while(n[++l] == n[l - 1] && l < r);
                    continue;
                }
                if (s > t) {
                    resMax = min(resMax, s);
                    while(n[--r] == n[r + 1] && l < r);
                    continue;
                }
                
                return t;
            }
        }
        if (abs(resMin - t) > abs(resMax - t)) return resMax;
        return resMin;
    }
};
