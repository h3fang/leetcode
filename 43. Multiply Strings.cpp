#include <algorithm>
#include <cstdio>
#include <string>

using namespace std;

class Solution {
public:
    string add(string num1, string num2) {
        string r;
        int carry = 0;
        for (int i = num1.size() - 1, j = num2.size() - 1; i >= 0 || j >= 0; i--, j--) {
            char c = carry;
            if (i >= 0) {
                c += num1[i] - '0';
            }
            if (j >= 0) {
                c += num2[j] - '0';
            }
            r.push_back(c % 10 + '0');
            carry = c > 9;
        }
        if (carry) {
            r.push_back('1');
        }
        reverse(r.begin(), r.end());
        return r;
    }

    string multiply(string num1, char c) {
        string r;
        int carry = 0;
        for (int i = num1.size() - 1; i >= 0; i--) {
            char n = (num1[i] - '0') * (c - '0') + carry;
            if (n > 9) {
                r.push_back(n % 10 + '0');
                carry = n / 10;
            } else {
                r.push_back(n + '0');
                carry = 0;
            }
        }
        if (carry) {
            r.push_back(carry + '0');
        }
        reverse(r.begin(), r.end());
        return r;
    }

    string multiply_slow(string num1, string num2) {
        string r;
        if (num2.size() > num1.size()) {
            swap(num1, num2);
        }
        if (num2 == "0") {
            return "0";
        }
        for (int i = num2.size() - 1; i >= 0; i--) {
            string t = multiply(num1, num2[i]) + string(num2.size() - 1 - i, '0');
            r = add(r, t);
        }
        return r;
    }

    string multiply(string num1, string num2) {
        string r(num1.size() + num2.size(), '0');
        for (int i = num1.size() - 1; i >= 0; i--) {
            int carry = 0;
            for (int j = num2.size() - 1; j >= 0; j--) {
                int n = r[i + j + 1] - '0' + (num1[i] - '0') * (num2[j] - '0') + carry;
                r[i + j + 1] = n % 10 + '0';
                carry = n / 10;
            }
            r[i] = carry + '0';
        }
        auto p = r.find_first_not_of('0');
        if (p != string::npos) {
            return r.substr(p);
        }
        return "0";
    }
};

int main() {
    string num1 = "123";
    string num2 = "456";
    printf("%s\n", Solution().multiply(num1, num2).data());
    return 0;
}