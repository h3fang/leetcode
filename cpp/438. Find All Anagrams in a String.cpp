#include <vector>
#include <string>
#include <set>
#include <algorithm>
#include <cstdio>

using namespace std;

class Solution {
public:
    bool check(int *t, int *ts) {
        for (int i=0; i<26; i++) {
            if (t[i] != ts[i]) {
                return false;
            }
        }
        return true;
    }

    vector<int> findAnagrams(string s, string p) {
        const int N = s.size(), M = p.size();
        int t[26] = {0}, ts[26] = {0};
        for (auto c:p) {
            ts[c-'a']++;
        }
        vector<int> r;
        if ( M == 0 || N == 0 || N < M) {return r;}
        for (int j=0; j<M; j++) {
            t[s[j]-'a']++;
        }
        if (check(t, ts)) {r.push_back(0);}
        t[s[0]-'a']--;

        for (int i=1; i<N; i++) {
            if (N-i < M) break;
            t[s[i+M-1]-'a']++;
            if (check(t, ts)) {r.push_back(i);}
            t[s[i]-'a']--;
        }
        return r;
    }
};

int main() {
    for (auto e : Solution().findAnagrams("baa", "aa")) {
        printf("%d ", e);
    }
    printf("\n");
    return 0;
}