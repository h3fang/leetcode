#include <algorithm>
#include <vector>
#include <string>
#include <cstdio>

using namespace std;

class Solution {
public:
    string frequencySort(string s) {
        vector<int> t(256, 0);
        for (auto c : s) {
            t[c] += 1;
        }
        vector<pair<char, int>> alphabets;
        for (int i=0; i<t.size(); i++) {
            if (t[i]>0) {
                alphabets.push_back(pair<char, int>(i, t[i]));
            }
        }
        sort(alphabets.begin(), alphabets.end(), [](auto& a, auto& b) {
            return a.second > b.second;
        });
        string r;
        for (auto a : alphabets) {
            r += string(a.second, a.first);
        }
        return r;
    }
};

int main() {
    string s = "tree";
    printf("%s\n", Solution().frequencySort(s).data());
    return 0;
}