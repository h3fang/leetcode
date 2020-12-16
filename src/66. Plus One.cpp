#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        for (int i = digits.size()-1; i>=0; i--) {
            digits[i] += 1;
            if (digits[i] == 10) {
                digits[i] = 0;
                if (i == 0) {
                    digits.insert(digits.begin(), 1);
                    break;
                }
            } else {
                break;
            }
        }
        return digits;
    }
};

int main() {
    vector<int> digits = {9,9};
    printf("{");
    for (int d : Solution().plusOne(digits)) {
        printf("%d,", d);
    }
    printf("\b}\n");
    return 0;
}