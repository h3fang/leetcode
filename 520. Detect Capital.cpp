#include <cstdio>
#include <string>

using namespace std;

class Solution {
public:
    bool detectCapitalUse(string word) {
        if (word.size() < 2) {
            return true;
        }
        const bool c_first = isupper(word[0]), c_seond = isupper(word[1]);
        if (!c_first && c_seond) {
            return false;
        }
        for (int i=2; i<word.size(); i++) {
            if (bool(isupper(word[i])) != c_seond) {
                return false;
            }
        }
        return true;
    }
};

int main() {
    string word("FlaG");
    printf("%s\n", Solution().detectCapitalUse(word) ? "true" : "false");
    return 0;
}