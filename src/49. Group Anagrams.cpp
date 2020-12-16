#include <cstdio>
#include <vector>
#include <string>
#include <algorithm>
#include <unordered_map>

using namespace std;

class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        unordered_map<string, vector<string>> counts;
        for (const auto& s : strs) {
            string key = s;
            sort(key.begin(), key.end());
            counts[key].push_back(s);
        }
        vector<vector<string>> r;
        for (const auto& [k,v] : counts) {
            r.push_back(v);
        }
        return r;
    }
};

int main() {
    vector<string> strs = {"eat","tea","tan","ate","nat","bat"};
    for (const auto& v : Solution().groupAnagrams(strs)) {
        for (const auto& s : v) {
            printf("%s, ", s.data());
        }
        printf("\b\b  \n");
    }
    return 0;
}