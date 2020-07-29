#include <algorithm>
#include <cstdio>
#include <string>

using namespace std;

class Solution {
public:
    string addBinary(string a, string b) {
        string r;
        char carry = 0;

        for (int i = a.size() - 1, j = b.size() - 1; i >= 0 || j >= 0; i--, j--) {
            char c = carry;
            if (i >= 0) {
                c += a[i] - '0';
            }
            if (j >= 0) {
                c += b[j] - '0';
            }
            if (c >= 2) {
                r.push_back(c - 2 + '0');
                carry = 1;
            } else {
                r.push_back(c + '0');
                carry = 0;
            }
        }

        if (carry) {
            r.push_back('1');
        }

        reverse(r.begin(), r.end());

        return r;
    }
};

int main() {
    string a = "1010", b = "1011";
    printf("%s\n", Solution().addBinary(a, b).data());
    return 0;
}
