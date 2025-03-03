#include <array>
#include <cassert>
#include <string>

using namespace std;

int index(char c) {
    if (c >= 'a' && c <= 'z') {
        return c - 'a';
    } else if (c >= 'A' && c <= 'Z') {
        return 26 + c - 'A';
    } else {
        return -1;
    }
}

class Solution {
public:
    string minWindow(string s, string t) {
        int m = s.size(), n = t.size();
        int l = 0, r = 0, min = m + 1, ans = 0;
        array<int, 52> f = {0};
        for (char c : t) {
            f[index(c)] += 1;
        }
        int unmatched = 0;
        for (int c : f) {
            if (c > 0) {
                unmatched += 1;
            }
        }
        while (r < m) {
            while (r < m && unmatched > 0) {
                int i = index(s[r]);
                f[i] -= 1;
                if (f[i] == 0) {
                    unmatched -= 1;
                }
                r += 1;
            }
            while (unmatched == 0 && l < r) {
                if (min > r - l) {
                    min = r - l;
                    ans = l;
                }
                int i = index(s[l]);
                f[i] += 1;
                if (f[i] == 1) {
                    unmatched += 1;
                }
                l += 1;
            }
        }
        return min == m + 1 ? "" : s.substr(ans, min);
    }
};

int main() {
    auto r = Solution().minWindow("ADOBECODEBANC", "ABC");
    assert(r == "BANC");
    return 0;
}