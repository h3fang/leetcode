#include <cstdio>
#include <string>
#include <cctype>

using namespace std;

class Solution {
public:
    int calculate(string s) {
        int r = 0, prev = 0;
        char op = '+';
        int num = -1;
        for (int i = 0; i <= s.size(); i++) {
            const char c = s[i];
            if (isdigit(c)) {
                if (num == -1) {
                    num = c - '0';
                }
                else {
                    num = num * 10 + (c - '0');
                }
            }
            else {
                if (num >= 0) {
                    int op1;
                    switch (op)
                    {
                    case '+':
                        r += num;
                        prev = num;
                        break;
                    case '-':
                        r += -num;
                        prev = -num;
                        break;
                    case '*':
                        r -= prev;
                        r += prev * num;
                        prev = prev * num;
                        break;
                    case '/':
                        r -= prev;
                        r += prev / num;
                        prev = prev / num;
                        break;
                    default:
                        break;
                    }
                    num = -1;
                }

                if (c == '+' || c == '-' || c == '*' || c == '/') {
                    op = c;
                }
            }
        }

        return r;
    }
};

int main() {
    string s = "421";
    printf("%d\n", Solution().calculate(s));
    return 0;
}
