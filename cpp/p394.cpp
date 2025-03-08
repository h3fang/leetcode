#include <cassert>
#include <string>

using namespace std;

class Solution {
    int i = 0;

public:
    string decode(string &s) {
        if (i == s.size()) {
            return "";
        }
        string r;
        while (i < s.size()) {
            if (isdigit(s[i])) {
                int n = 0;
                while (isdigit(s[i])) {
                    n = n * 10 + s[i] - '0';
                    i += 1;
                }
                i += 1;
                auto sub = decode(s);
                for (int j = 0; j < n; j++) {
                    r += sub;
                }
            } else if (s[i] == ']') {
                i += 1;
                break;
            } else {
                r.push_back(s[i]);
                i += 1;
            }
        }
        return r;
    }

    string decodeString(string s) {
        return decode(s);
    }
};

int main() {
    auto r = Solution().decodeString("3[a2[c]]");
    assert("accaccacc" == r);
    return 0;
}
