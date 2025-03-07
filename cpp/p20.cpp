#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    bool isValid(string s) {
        vector<char> q;
        for (char c : s) {
            if (c == '{' || c == '[' || c == '(') {
                q.push_back(c);
            } else {
                if (q.empty()) {
                    return false;
                }
                char b = q.back();
                q.pop_back();
                if (c > b + 2 || c <= b) {
                    return false;
                }
            }
        }
        return q.empty();
    }
};

int main() {
    assert(Solution().isValid("()"));
    assert(Solution().isValid("()[]{}"));
    assert(!Solution().isValid("(]"));
    assert(Solution().isValid("([])"));
    return 0;
}