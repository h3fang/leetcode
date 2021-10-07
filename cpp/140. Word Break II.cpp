#include <cstdio>
#include <string>
#include <vector>
#include <unordered_set>
#include <unordered_map>

using namespace std;

class Solution {
    unordered_map<string, vector<string>> memo;

public:
    vector<string> wordBreak(string s, vector<string>& wordDict) {
        unordered_set<string> t;
        for (const auto& w : wordDict) {
            t.insert(w);
        }
        return helper(s, t);
    }

    vector<string> helper(string s, const unordered_set<string>& t) {
        if (memo.count(s)) {
            return memo[s];
        }
        vector<string> result;
        if(t.count(s)){
            result.push_back(s);
        }
        for (int i = 1; i < s.size(); i++) {
            const string word = s.substr(0, i);
            if (t.count(word)) {
                for (const auto& sentence : helper(s.substr(i), t)) {
                    result.insert(result.end(), word + " " + sentence);
                }
            }
        }
        memo[s] = result;
        return result;
    }
};

int main() {
    string s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaa"
               "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    vector<string> wordDict = {"a",      "aa",      "aaa",      "aaaa",      "aaaaa",
                               "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"};
    // string s = "catsanddog";
    // vector<string> wordDict = {"cat","cats","and","sand","dog"};
    for (const auto &st : Solution().wordBreak(s, wordDict)) {
        printf("%s\n", st.data());
    }
    return 0;
}
