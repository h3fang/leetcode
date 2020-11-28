#include <cstdio>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int longestSubstring(string s, int k) {
        const int n = s.size();
        return dc(s, 0, n, k);
    }

    int dc(const string &s, int start, int end, int k) {
        if (end < k) {
            return 0;
        }
        int count[26] = {0};
        for (int i = start; i < end; i++) {
            count[s[i] - 'a']++;
        }
        for (int mid = start; mid < end; mid++) {
            if (count[s[mid] - 'a'] >= k) {
                continue;
            }
            int nextStart = mid + 1;
            for (; nextStart < end; nextStart++) {
                if (count[s[nextStart] - 'a'] >= k) {
                    break;
                }
            }
            return max(dc(s, start, mid, k), dc(s, nextStart, end, k));
        }
        return end - start;
    }
};

int main() {
    string s = "aaabb";
    const int k = 3;
    printf("%d\n", Solution().longestSubstring(s, k));
    return 0;
}
