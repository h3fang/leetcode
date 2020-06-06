#include <string>
#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    void reverseString(vector<char>& s) {
        for (int i=0, j=s.size()-1; i<j; i++, j--) {
            swap(s[i], s[j]);
        }
    }
};

int main() {
    string str = "hello world!";
    vector<char> s(str.begin(), str.end());
    Solution().reverseString(s);
    printf("%s\n", string(s.begin(), s.end()).data());
    return 0;
}