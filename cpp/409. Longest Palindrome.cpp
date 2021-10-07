#include <cstdio>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int longestPalindrome(string s) {
        unordered_map<char, int> freq;
        for (char c : s) {
            freq[c]++;
        }
        int r = 0;
        for (auto [k,v] : freq) {
            r += v / 2 * 2;
            if (r % 2 == 0 && v % 2 == 1) {
                r++;
            }
        }
        return r;
    }
};

int main() {
    printf("%d\n", Solution().longestPalindrome("abccccdd"));
    return 0;
}