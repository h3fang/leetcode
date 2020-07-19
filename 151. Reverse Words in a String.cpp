#include <cstdio>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    string reverseWords(string s) {
        int a = -1, b = -1;
        vector<string> words;
        for (int i = 0; i <= s.size(); i++) {
            if (i == s.size() || s[i] == ' ') {
                if (a > -1) {
                    words.push_back(s.substr(a, b-a+1));
                    a = -1;
                }
            } else {
                if (a == -1) {
                    a = i;
                }
                b = i;
            }
        }
        string r;
        for (int i=words.size()-1; i>=0; i--) {
            r += words[i] + " ";
        }
        if (r.size()) {
            r.pop_back();
        }
        return r;
    }
};

int main() {
    printf("%s\n", Solution().reverseWords("hello world").data());
    return 0;
}