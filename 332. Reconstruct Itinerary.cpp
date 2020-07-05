#include <cstdio>
#include <vector>
#include <string>
#include <unordered_map>
#include <algorithm>

using namespace std;

class Solution {
public:
    bool bt(unordered_map<string, vector<string>>& m, vector<string>& r, const string& c, const int N) {
        auto& s = m[c];
        for (int i = 0; i < s.size(); i++) {
            string n = s[i];
            r.push_back(n);
            s.erase(s.begin()+i);
            if (!bt(m, r, n, N)) {
                s.insert(s.begin()+i, n);
                r.pop_back();
            } else {
                return true;
            }
        }
        return r.size() == N;
    }

    vector<string> findItinerary(vector<vector<string>>& tickets) {
        const int N = tickets.size()+1;
        unordered_map<string, vector<string>> m;
        for (auto& t : tickets) {
            m[t[0]].push_back(t[1]);
        }
        for (auto& [k,v] : m) {
            sort(v.begin(), v.end());
        }
        string s = "JFK";
        vector<string> r{s};
        bt(m, r, s, N);
        return r;
    }
};

int main() {
    vector<vector<string>> tickets = {{"JFK","SFO"},{"JFK","ATL"},{"SFO","ATL"},{"ATL","JFK"},{"ATL","SFO"}};
    for (auto& s : Solution().findItinerary(tickets)) {
        printf("%s ", s.data());
    }
    printf("\n");
    return 0;
}