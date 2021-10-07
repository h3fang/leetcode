#include <algorithm>
#include <cmath>
#include <cstdio>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int dfs(const vector<int> &choices, const vector<int> &ds, int i) {
        if (i >= ds.size()) {
            return 1;
        }

        bool equal = false;
        int smaller = 0;

        for (int n : choices) {
            if (n == ds[i]) {
                equal = true;
                break;
            } else if (n < ds[i]) {
                smaller++;
            } else {
                break;
            }
        }

        int r = smaller * pow(choices.size(), (ds.size() - i - 1));
        if (equal) {
            r += dfs(choices, ds, i + 1);
        }

        return r;
    }

    int atMostNGivenDigitSet(vector<string> &digits, int n) {
        vector<int> ds, choices;
        while (n > 0) {
            ds.push_back(n % 10);
            n /= 10;
        }

        reverse(ds.begin(), ds.end());

        for (auto &s : digits) {
            choices.push_back(s[0] - '0');
        }

        sort(choices.begin(), choices.end());

        int r = 0;
        for (int i = 1; i < ds.size(); i++) {
            r += pow(choices.size(), i);
        }

        return r + dfs(choices, ds, 0);
    }
};

int main() {
    vector<string> digits = {"1"};
    const int n = 843;
    printf("%d\n", Solution().atMostNGivenDigitSet(digits, n));
    return 0;
}
