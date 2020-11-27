#include <cctype>
#include <cstdio>
#include <string>

using namespace std;

class Solution {
public:
    string recursive(string& s, int start, int end) {
        if (start >= end) {
            return "";
        }
        string left, right;
        while (start < end) {
            if (isalpha(s[start])) {
                left += s[start];
                start++;
            }
            else {
                break;
            }
        }

        if (start == end) {
            return left;
        }

        int num = 0;
        while (start < end) {
            if (isdigit(s[start])) {
                num = num * 10 + s[start] - '0';
                start++;
            }
            else {
                break;
            }
        }
        start++;
        int new_end = start;
        int left_bracket = 1;
        while (true) {
            if (s[new_end] == '[') {
                left_bracket++;
            }
            else if (s[new_end] == ']') {
                left_bracket--;
            }
            if (left_bracket == 0) {
                break;
            }
            new_end++;
        }
        string encoded = recursive(s, start, new_end);
        for (int i = 0; i < num; i++) {
            left += encoded;
        }
        if (new_end+1 < end) {
            left += recursive(s, new_end+1, end);
        }
        return left;
    }

    string decodeString(string s) {
        return recursive(s, 0, s.size());
    }
};

int main() {
    string str = "3[a2[c]]";
    printf("%s\n", Solution().decodeString(str).data());
    return 0;
}
